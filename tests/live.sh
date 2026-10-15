#!/usr/bin/env bash
# end-to-end checks against a running pankat on a scratch copy of documents/: file serving,
# 404, admin login, drafts, special pages and the live updates via websocket
#
# usage: nix develop -c just test-live
# needs: a built pankat (PANKAT_BIN, default target/debug/pankat), pandoc, curl and websocat
# (taken from nixpkgs when it's not in PATH)
set -u

ROOT=$(cd "$(dirname "$0")/.." && pwd)
BIN=${PANKAT_BIN:-$ROOT/target/debug/pankat}
PORT=${PANKAT_TEST_PORT:-5057}
WORK=$ROOT/target/live-test
POSTS=$WORK/documents/blog.lastlog.de/posts/livetest
PASSWORD=test-password
PID=
FAILED=0
PASSED=0

if command -v websocat >/dev/null; then
  WEBSOCAT=(websocat)
else
  WEBSOCAT=(nix shell nixpkgs#websocat -c websocat)
fi

check() { # description expected actual
  if [ "$2" = "$3" ]; then
    PASSED=$((PASSED + 1))
    echo "  ok    $1"
  else
    FAILED=$((FAILED + 1))
    echo "  FAIL  $1: expected '$2', got '$3'"
  fi
}

contains() { # file pattern -> 1 or 0
  grep -q -- "$2" "$1" 2>/dev/null && echo 1 || echo 0
}

start() { # subdir
  SUBDIR=$1
  BASE=http://127.0.0.1:$PORT$SUBDIR
  git -C "$ROOT" show HEAD:pankat.toml | sed \
    -e "s|^port = .*|port = $PORT|" \
    -e "s|^subdir = .*|subdir = \"$SUBDIR\"|" \
    -e "s|^jwt_token = .*|jwt_token = \"test-signing-secret\"|" \
    -e "s|^admin_password = .*|admin_password = \"$PASSWORD\"|" >"$WORK/pankat.toml"
  (cd "$WORK" && exec "$BIN" >"$WORK/log" 2>&1) &
  PID=$!
  for _ in $(seq 1 180); do
    grep -q Listening "$WORK/log" 2>/dev/null && return
    kill -0 "$PID" 2>/dev/null || break
    sleep 1
  done
  echo "pankat did not start:"
  tail -20 "$WORK/log"
  exit 1
}

stop() {
  [ -n "$PID" ] && kill "$PID" 2>/dev/null && wait "$PID" 2>/dev/null
  PID=
}
trap stop EXIT

status() { # curl args... -> http status
  curl -s --path-as-is -o "$WORK/body" -w '%{http_code}' "$@"
}

login() { # -> cookie "pankat_session=..."
  curl -s -D - -o /dev/null --data-urlencode "password=$PASSWORD" "$BASE/login" |
    grep -i '^set-cookie' | sed 's/^[^:]*: //; s/;.*//'
}

listen() { # outfile seconds src_file_name [cookie] -> websocket messages without pings, background
  local header=()
  [ -n "${4:-}" ] && header=(-H "Cookie: $4")
  { echo "{ \"register\": \"$3\" }"; sleep "$2"; } |
    timeout $(($2 + 2)) "${WEBSOCAT[@]}" "ws://127.0.0.1:$PORT$SUBDIR/api/ws" "${header[@]}" 2>/dev/null |
    grep -v '"ping"' >"$1" &
}

article() { # name date [plugins...] -> writes posts/livetest/<name>.mdwn
  local name=$1 date=$2
  shift 2
  { echo "[[!meta date=\"$date\"]]"; printf '%s\n' "$@"; echo; echo "body of $name"; } >"$POSTS/$name.mdwn"
}

settle() { sleep 3; } # the file monitor debounces and rebuilds

echo "preparing $WORK"
rm -rf "$WORK" && mkdir -p "$WORK" && cp -r "$ROOT/documents" "$WORK/"
rm -rf "$POSTS" && mkdir -p "$POSTS"

for SUBDIR in "" "/blog"; do
  echo "file serving and 404, subdir='$SUBDIR'"
  start "$SUBDIR"
  for path in /../../pankat.toml /posts/../../../pankat.toml /assets/../../pankat.toml; do
    check "traversal $path" "404 0" "$(status "$BASE$path") $(contains "$WORK/body" admin_password)"
  done
  check "missing page shows the 404 page" "404 1" \
    "$(status "$BASE/does-not-exist.html") $(contains "$WORK/body" 'page not found')"
  check "404 page links to the index" 1 "$(contains "$WORK/body" "href=\"$SUBDIR/index.html\"")"
  check "index" 200 "$(status "$BASE/index.html")"
  check "subdir root with slash" 200 "$(status "$BASE/")"
  check "asset" 200 "$(status "$BASE/assets/css/style.css")"
  check "outside of subdir" 404 "$(status "http://127.0.0.1:$PORT/outside")"
  stop
done

start "/blog"

echo "admin login"
check "session without cookie" '{"admin":false}' "$(curl -s "$BASE/api/session")"
check "login page" "200 1" "$(status "$BASE/login") $(contains "$WORK/body" 'single user')"
check "wrong password" 401 "$(status --data-urlencode password=wrong "$BASE/login")"
COOKIE=$(login)
check "session cookie set" 1 "$([ -n "$COOKIE" ] && echo 1 || echo 0)"
check "session with cookie" '{"admin":true}' "$(curl -s -b "$COOKIE" "$BASE/api/session")"
check "foreign next is ignored" "$BASE/drafts/" "$(curl -s -o /dev/null -w '%{redirect_url}' \
  --data-urlencode "password=$PASSWORD" --data-urlencode 'next=//evil.example/' "$BASE/login")"
check "logout clears the cookie" 1 "$(curl -s -D - -o /dev/null -X POST -b "$COOKIE" "$BASE/logout" |
  grep -ci '^set-cookie: pankat_session=;.*Max-Age=0')"

echo "drafts and live updates"
check "drafts need a login" "303 $BASE/login?next=%2Fblog%2Fdrafts%2F" \
  "$(curl -s -o /dev/null -w '%{http_code} %{redirect_url}' "$BASE/drafts/")"
article demo "2026-10-12 12:00"
settle
listen "$WORK/ws_admin" 14 posts/livetest/demo.mdwn "$COOKIE"
listen "$WORK/ws_public" 14 posts/livetest/demo.mdwn
sleep 3
article demo "2026-10-12 12:00" "[[!draft]]"
settle
check "draft not in the output" "0 404" \
  "$([ -f "$WORK/documents/output/demo.html" ] && echo 1 || echo 0) $(status "$BASE/demo.html")"
check "draft listed for the admin" 1 "$(curl -s -b "$COOKIE" "$BASE/drafts/" | grep -c 'drafts/demo.html')"
check "draft page for the admin" 200 "$(status -b "$COOKIE" "$BASE/drafts/demo.html")"
check "draft page needs a login" 303 "$(status "$BASE/drafts/demo.html")"
article demo "2026-10-12 12:00" "[[!draft]]" "edited draft"
settle
article demo "2026-10-12 12:00"
settle
wait
check "republished in the output" 1 "$([ -f "$WORK/documents/output/demo.html" ] && echo 1 || echo 0)"
check "admin: turned into a draft" 1 "$(contains "$WORK/ws_admin" 'article turned into a draft')"
check "admin: live draft edit" 1 "$(contains "$WORK/ws_admin" 'edited draft')"
check "admin: published" 1 "$(contains "$WORK/ws_admin" 'draft published')"
check "public: removed" 1 "$(contains "$WORK/ws_public" 'article removed')"
check "public: no draft content" 0 "$(contains "$WORK/ws_public" 'edited draft')"
check "public: published" 1 "$(contains "$WORK/ws_public" 'draft published')"

echo "rename and remove"
listen "$WORK/ws_rename" 6 posts/livetest/demo.mdwn
sleep 2
mv "$POSTS/demo.mdwn" "$POSTS/renamed.mdwn"
settle
wait
check "old page shows the new location" 1 "$(contains "$WORK/ws_rename" 'href=\\"/blog/renamed.html\\"')"
check "old html removed" 0 "$([ -f "$WORK/documents/output/demo.html" ] && echo 1 || echo 0)"
check "new html written" 1 "$(contains "$WORK/documents/output/renamed.html" '<title>renamed')"
listen "$WORK/ws_remove" 6 posts/livetest/renamed.mdwn
sleep 2
rm "$POSTS/renamed.mdwn"
settle
wait
check "removed page links to the index" 1 "$(contains "$WORK/ws_remove" 'article removed')"
check "removed html deleted" 0 "$([ -f "$WORK/documents/output/renamed.html" ] && echo 1 || echo 0)"

echo "series"
article first "2026-10-11 12:00" "[[!series livetestseries]]"
article second "2026-10-12 12:00" "[[!series livetestseries]]"
settle
check "series navigation" 1 "$(contains "$WORK/documents/output/second.html" seriesNav)"
article second "2026-10-12 12:00"
settle
check "series navigation gone after removing [[!series]]" 0 "$(contains "$WORK/documents/output/second.html" seriesNav)"
check "former series neighbour doesn't link to it" 0 "$(contains "$WORK/documents/output/first.html" 'second.html')"

echo "special pages"
article secret_special "2026-10-01 12:00" "[[!SpecialPage]]" "[[!draft]]"
settle
check "special pages need a login" 303 "$(status "$BASE/specialpages/")"
check "special pages for the admin" 200 "$(status -b "$COOKIE" "$BASE/specialpages/")"
check "special page listed" 1 "$(contains "$WORK/body" 'href="/blog/about.html"')"
check "draft special page links to drafts/" 1 "$(contains "$WORK/body" 'href="/blog/drafts/secret_special.html"')"
check "draft special page not public" 404 "$(status "$BASE/secret_special.html")"
stop

echo "startup removes the stale html of a draft"
article offline "2026-10-01 12:00" "[[!draft]]"
echo "stale public copy" >"$WORK/documents/output/offline.html"
start "/blog"
check "stale html removed" "0 404" \
  "$([ -f "$WORK/documents/output/offline.html" ] && echo 1 || echo 0) $(status "$BASE/offline.html")"
check "no panic in the log" 0 "$(contains "$WORK/log" panicked)"
stop

echo
echo "$PASSED passed, $FAILED failed"
[ "$FAILED" -eq 0 ] && rm -rf "$WORK"
[ "$FAILED" -eq 0 ]
