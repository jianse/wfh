let s = 'ExportManifest.json!00_w5hQItnM-1bKQNJUkXbcog'

s = s.replace('Export', '')

console.log(s)

s = s.replace(/(_[a-z]{2})?\.json.*/, '')

console.log(s)
