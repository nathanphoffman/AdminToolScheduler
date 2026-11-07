Thank you for trying our NEW WEB!

How To Run
-----------------------
If you are seeing this file you have likely ran a git command to copy this directory, if so
then all you need to do is run "npm run start" to start the web server.

Basic Usage
-----------------------
The markdown that is your homepage is 'main.md'  If you create other markdown files 
in the same directory as main.md, they will be navigatable as localhost#{filename}.md

You can link to markdown files in the same directory by just putting the markdown path in
a link like: [About this project](about)  This will load about.md in your root. If it is in 
a subdirectory you can just reference it like info/about which loads info/about.md

If linking to sites off of your domain that use the markdown protcol you don't need 
anything special since this is new-web forward. So NatesSuperAwesomeBlog.com will load 
great if it has a main.md and supports the new-web markdown syntax.

Http/Https links work (in newweb browsers they will open an external application) in the 
web mode they will open inside the web browser you are running it in.  However, you must 
always specify https:// or http:// if you are loading an http resource.  google.com will 
assume it is a markdown site, you will get a warning about this on localhost.
