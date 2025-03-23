A ***prototype note*** is one that represents a <u>standardized note type</u> from which instances can be created. They will normally have <u>accompanying templates</u> for ease of creating.
- You can create an instance of a prototype note by specifying the `proto` property
- You can create a new prototype note by specifying `proto: "[[Prototype note]]"`

These ***prototype notes*** are designed to introduce *(and aid with)* regularity of creating *"auxiliary"* notes to normal notes; e.g. notes like "Glossary of \<topic\>" can all be instances of ***prototype note*** [[Glossary of|Glossary of]].

# Prototype hierarchy
```breadcrumbs
type: tree 
fields: [proto-instances]
show-attributes: [field] 
sort: field
dataview-from: '!"templates"'
```


