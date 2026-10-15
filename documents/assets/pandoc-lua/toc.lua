-- static table of contents, replaces jquery.tocify: the article title (pandoc -M pankat-title=...) is the root,
-- the sections are nested below it. runs after shifted-numbered-headings.lua, so the title stays the only h1
-- and ## / ### are the h2 / h3 which tocify listed. written hidden into the article, toc.js copies it into
-- #toc of the standalone template (outside #container, so it can slide behind it) and keeps it in sync on
-- live updates
function Pandoc(doc)
  local sections = pandoc.structure.table_of_contents(doc.blocks, { toc_depth = 3 })
  if #sections.content == 0 then
    return doc
  end
  local title = pandoc.utils.stringify(doc.meta['pankat-title'] or '')
  local root = pandoc.BulletList({ { pandoc.Plain({ pandoc.Link(title, '#SiteTitle') }), sections } })
  table.insert(doc.blocks, 1, pandoc.Div({ root }, pandoc.Attr('toc-source', {}, { { 'hidden', '' } })))
  return doc
end
