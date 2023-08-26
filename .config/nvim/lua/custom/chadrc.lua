---@type ChadrcConfig 
 local M = {}
 M.ui = {
  theme = 'monekai',
  transparency = true,
  changed_themes = {
      monekai = {
         base_30 = {
            grey = "#57ff00",
            grey_fg = "#de79ff",
            white = "#ffec00",
         },
      }
  }
 }
 M.plugins = 'custom.plugins'
 return M

