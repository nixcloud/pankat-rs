function Pandoc(doc)
-- Create and number sections
doc.blocks = pandoc.utils.make_sections(true, nil, doc.blocks)

-- Table to track heading IDs for unique anchors
local heading_id_counts = {}

-- Process headers and add anchors
doc.blocks = doc.blocks:walk {
Header = function(h)
  -- Create a base ID or use the existing one
  local base_id = h.identifier ~= '' and h.identifier or pandoc.utils.stringify(h.content)
  base_id = string.gsub(base_id, '[^%w%-]+', '-') -- Sanitize

  -- Generate unique ID by appending count if necessary
  local count = heading_id_counts[base_id] or 0
  local unique_id = base_id
  if count > 0 then
    unique_id = base_id .. '-' .. tostring(count)
  end
  heading_id_counts[base_id] = count + 1

  -- Set the header's ID
  h.identifier = unique_id

  -- Create the anchor as a RawInline and insert it into the header content
  local anchor_html = pandoc.RawInline('html', '<a class="glyphicon glyphicon-link" aria-label="Anchor"  href="#' .. unique_id .. '" style="font-size: medium; font-style: normal; font-variant: normal; font-weight: normal; line-height: 1; padding-left: 0.375em; vertical-align: middle;"></a>')

  -- Insert the anchor into the header's content
  table.insert(h.content, anchor_html)

  -- Return the modified header
  return h
end
}

-- Return the modified document
return doc
end