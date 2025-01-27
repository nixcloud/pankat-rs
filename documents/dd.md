[[!meta date="2015-06-03 16:58"]]
[[!tag nixos go webkit gtk]]
[[!summary webloop on nixos porting attempt]]
[[!img media/nixos-lores.png alt="" style="float: right" class="noFancy"]]

# motivation
i wanted to package webloop on nixos and since the software is kind of broken i can't complete that task but if someone wants to look into it, here is what i've done so far.

# current status
on [1] i've packaged the following software:

* webkitgtk-2.8.3
* gojs
* gotk2
* go-webkit2

the main issue i'm facing right now is that i'm forcing gojs/go-webkit2:

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ {.bash}
gojs:
sed -i 's,javascriptcoregtk-3.0,javascriptcoregtk-4.0,g'  go/src/github.com/sqs/gojs/base.go

go-webkit2:
sed -i 's,webkit2gtk-3.0,webkit2gtk-4.0,g' go/src/github.com/sourcegraph/go-webkit2/webkit2/webview.go
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

into using javascriptcoregtk-4.0 and webkit2gtk-4.0 for which they were not designed. it might be not much work to update gojs / go-webkit2 to make proper use of these libraries but i can't do that at the moment.

the bug report is at [2].

# conclusion
some final works on go on nixos: i'm loving it:

* **go makes vast use of git repositories** (altough i would wish authors to create more releases/tags instead of just using any random git revision number)
* **go only requires GOPATH**
* **go emphasises unit tests** and has a very clean and easy to use methology
* **go 1.3 and 1.4** are already packaged for nixos and are very usable already

**i would recommend everyone to look into go on nixos** as it seems to be a great fit on nixos. this holds also true if you use nix on other POSIX platforms like mac os x or basically any other linux.

thanks to lethalman as he was helping me so much with getting into go packaging!

# links

* [1] <https://github.com/qknight/nixpkgs/tree/webloop>
* [2] <https://github.com/sourcegraph/go-webkit2/issues/20>
