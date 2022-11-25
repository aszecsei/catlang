"use strict";(self.webpackChunk=self.webpackChunk||[]).push([[935],{3905:(e,t,r)=>{r.d(t,{Zo:()=>d,kt:()=>u});var a=r(7294);function n(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,a)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){n(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,a,n=function(e,t){if(null==e)return{};var r,a,n={},o=Object.keys(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||(n[r]=e[r]);return n}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(n[r]=e[r])}return n}var l=a.createContext({}),c=function(e){var t=a.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},d=function(e){var t=c(e.components);return a.createElement(l.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},h=a.forwardRef((function(e,t){var r=e.components,n=e.mdxType,o=e.originalType,l=e.parentName,d=s(e,["components","mdxType","originalType","parentName"]),h=c(r),u=n,m=h["".concat(l,".").concat(u)]||h[u]||p[u]||o;return r?a.createElement(m,i(i({ref:t},d),{},{components:r})):a.createElement(m,i({ref:t},d))}));function u(e,t){var r=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var o=r.length,i=new Array(o);i[0]=h;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s.mdxType="string"==typeof e?e:n,i[1]=s;for(var c=2;c<o;c++)i[c]=r[c];return a.createElement.apply(null,i)}return a.createElement.apply(null,r)}h.displayName="MDXCreateElement"},4843:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>d,contentTitle:()=>l,default:()=>u,frontMatter:()=>s,metadata:()=>c,toc:()=>p});var a=r(7462),n=r(3366),o=(r(7294),r(3905)),i=["components"],s={title:"Roadmap",author:"Alic Szecsei",authorURL:"https://alic-szecsei.com/",authorImageURL:"https://alic-szecsei.com/img/profile2.png"},l=void 0,c={permalink:"/catlang/blog/2018/12/16/roadmap",editUrl:"https://github.com/aszecsei/catlang/edit/master/website/blog/2018-12-16-roadmap.md",source:"@site/blog/2018-12-16-roadmap.md",title:"Roadmap",description:"We're fast approaching the release of version 0.1! The lexer and parser are mostly complete, barring a few additional features. In this post, I'll talk about how far the compiler has come - and where continued efforts are going.",date:"2018-12-16T00:00:00.000Z",formattedDate:"December 16, 2018",tags:[],readingTime:2.95,hasTruncateMarker:!0,authors:[{name:"Alic Szecsei",url:"https://alic-szecsei.com/",imageURL:"https://alic-szecsei.com/img/profile2.png"}],frontMatter:{title:"Roadmap",author:"Alic Szecsei",authorURL:"https://alic-szecsei.com/",authorImageURL:"https://alic-szecsei.com/img/profile2.png"},nextItem:{title:"Hello, World!",permalink:"/catlang/blog/2018/10/17/hello-world"}},d={authorsImageUrls:[void 0]},p=[{value:"From Go To Rust",id:"from-go-to-rust",level:2},{value:"The Rust Conversion",id:"the-rust-conversion",level:2},{value:"The Lexer",id:"the-lexer",level:2},{value:"The Parser",id:"the-parser",level:2},{value:"Code Generation",id:"code-generation",level:2},{value:"The Lexer",id:"the-lexer-1",level:2},{value:"The Parser",id:"the-parser-1",level:2},{value:"Code Generation",id:"code-generation-1",level:2}],h={toc:p};function u(e){var t=e.components,r=(0,n.Z)(e,i);return(0,o.kt)("wrapper",(0,a.Z)({},h,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"We're fast approaching the release of version 0.1! The lexer and parser are mostly complete, barring a few additional features. In this post, I'll talk about how far the compiler has come - and where continued efforts are going."),(0,o.kt)("h1",{id:"progress"},"Progress"),(0,o.kt)("h2",{id:"from-go-to-rust"},"From Go To Rust"),(0,o.kt)("p",null,"After a brief false start in Go, catlang transitioned to use Rust. I put some effort into creating a small extension for Visual Studio Code to perform basic syntax highlighting. This helped when writing example code, as well as when ensuring that there wasn't much gap between the language grammar (written in EBNF) and the language specification detailed in this documentation."),(0,o.kt)("p",null,"I also transitioned the language documentation from using LaTeX to using docusaurus; this has the benefit of automatic updates to GitHub pages, enabling new users to more easily learn the language while maintaining better formatting than I achieved in LaTeX."),(0,o.kt)("h2",{id:"the-rust-conversion"},"The Rust Conversion"),(0,o.kt)("p",null,"At first, I tried to convert line-by-line from my Go code to Rust. This ended up frustrating me a bit - occasional issues with the borrow checker caused me to have to re-consider data ownership. Error handling, in particular, became much more complex. This led to a secondary overhaul, late in development, when I worked to add further functionality for error handling. To help with this, I was able to adapt some ideas from the Rust compiler."),(0,o.kt)("h2",{id:"the-lexer"},"The Lexer"),(0,o.kt)("p",null,"Most of the lexer was straightforward, and didn't require much alteration. Perhaps the only complex aspect was the few 3-character tokens (such as ",(0,o.kt)("inlineCode",{parentName:"p"},">>=")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"&&="),") and even this was easily resolved with nested conditionals."),(0,o.kt)("h2",{id:"the-parser"},"The Parser"),(0,o.kt)("p",null,"The parser was more complex - the EBNF grammar I'd written turned out at several points to not actually reflect the language documentation I'd been writing. There was something of a back-and-forth between the grammar and the parser, as I realized that aspects needed to be fixed. In addition, a rewrite was necessary partway through writing the expression parser, as I realized that all of the parsing I was doing was right-associative, and several operators (such as subtraction) needed to be left-associative instead."),(0,o.kt)("p",null,"In addition, figuring out how to perform error handling in Rust caused a few issues. I wanted to be able to display problematic spans of code, and so needed both a ",(0,o.kt)("inlineCode",{parentName:"p"},"peekable")," iterator (for the lexer) and a regular ",(0,o.kt)("inlineCode",{parentName:"p"},"String")," for the error reporting. Because a structure in Rust cannot hold a reference to one of its own members, I had to set up these in separate parts of the code, and pass them through as separate arguments."),(0,o.kt)("h2",{id:"code-generation"},"Code Generation"),(0,o.kt)("p",null,"I ran into a few issues with building LLVM instructions - the ",(0,o.kt)("inlineCode",{parentName:"p"},"llvm-sys")," package ran into linker errors on Windows, and so I had to pivot to generating C code instead. This has the beginning of functionality."),(0,o.kt)("p",null,"The code generation features don't quite compile yet, and so are behind a separate ",(0,o.kt)("inlineCode",{parentName:"p"},"c-codegen")," branch."),(0,o.kt)("h1",{id:"future-work"},"Future Work"),(0,o.kt)("h2",{id:"the-lexer-1"},"The Lexer"),(0,o.kt)("p",null,"The lexer still needs some functionality for dealing with floating point numbers, and numbers with non-10 bases. Aside from this, the lexer is mostly complete."),(0,o.kt)("h2",{id:"the-parser-1"},"The Parser"),(0,o.kt)("p",null,"A few statements still need implementation efforts - loops, branching statements, inner blocks and imports, and so on."),(0,o.kt)("p",null,"In addition, the created parse tree still needs to be knitted and type-checked."),(0,o.kt)("h2",{id:"code-generation-1"},"Code Generation"),(0,o.kt)("p",null,"While the C code generation works as a stopgap measure, determining the root cause of the ",(0,o.kt)("inlineCode",{parentName:"p"},"llvm-sys")," linker errors would likely be a better way to proceed. In addition, no optimization features are currently included."))}u.isMDXComponent=!0}}]);