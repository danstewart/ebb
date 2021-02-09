# Architecture


## Code Overview
```bash
.
└── src  # Root of the code
   ├── backends  # Backend storage logic
   ├── commands  # CLI sub commands
   ├── lib       # Misc code
   └── templates # File templates
```

## Commands
**init**
Initialise a new blog, ask the user for all the details we need and stores that in `.ebb-conf.json`.  
Also creates the default `wrapper.html` that all blog posts are wrapped in.  

**add**
Creates a new blog post.  
