"use strict";(self.webpackChunk=self.webpackChunk||[]).push([[575],{3905:(e,n,r)=>{r.d(n,{Zo:()=>u,kt:()=>f});var t=r(7294);function o(e,n,r){return n in e?Object.defineProperty(e,n,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[n]=r,e}function a(e,n){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var t=Object.getOwnPropertySymbols(e);n&&(t=t.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),r.push.apply(r,t)}return r}function i(e){for(var n=1;n<arguments.length;n++){var r=null!=arguments[n]?arguments[n]:{};n%2?a(Object(r),!0).forEach((function(n){o(e,n,r[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(r,n))}))}return e}function c(e,n){if(null==e)return{};var r,t,o=function(e,n){if(null==e)return{};var r,t,o={},a=Object.keys(e);for(t=0;t<a.length;t++)r=a[t],n.indexOf(r)>=0||(o[r]=e[r]);return o}(e,n);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(t=0;t<a.length;t++)r=a[t],n.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var l=t.createContext({}),s=function(e){var n=t.useContext(l),r=n;return e&&(r="function"==typeof e?e(n):i(i({},n),e)),r},u=function(e){var n=s(e.components);return t.createElement(l.Provider,{value:n},e.children)},p={inlineCode:"code",wrapper:function(e){var n=e.children;return t.createElement(t.Fragment,{},n)}},d=t.forwardRef((function(e,n){var r=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,u=c(e,["components","mdxType","originalType","parentName"]),d=s(r),f=o,g=d["".concat(l,".").concat(f)]||d[f]||p[f]||a;return r?t.createElement(g,i(i({ref:n},u),{},{components:r})):t.createElement(g,i({ref:n},u))}));function f(e,n){var r=arguments,o=n&&n.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=d;var c={};for(var l in n)hasOwnProperty.call(n,l)&&(c[l]=n[l]);c.originalType=e,c.mdxType="string"==typeof e?e:o,i[1]=c;for(var s=2;s<a;s++)i[s]=r[s];return t.createElement.apply(null,i)}return t.createElement.apply(null,r)}d.displayName="MDXCreateElement"},5538:(e,n,r)=>{r.r(n),r.d(n,{assets:()=>u,contentTitle:()=>l,default:()=>f,frontMatter:()=>c,metadata:()=>s,toc:()=>p});var t=r(7462),o=r(3366),a=(r(7294),r(3905)),i=["components"],c={id:"generics",title:"Generics",sidebar_label:"Generics"},l=void 0,s={unversionedId:"generics",id:"generics",title:"Generics",description:"Generics look similar to generics in other languages:",source:"@site/docs/generics.md",sourceDirName:".",slug:"/generics",permalink:"/catlang/docs/generics",draft:!1,editUrl:"https://github.com/aszecsei/catlang/edit/master/website/docs/generics.md",tags:[],version:"current",frontMatter:{id:"generics",title:"Generics",sidebar_label:"Generics"},sidebar:"docs",previous:{title:"Casting",permalink:"/catlang/docs/casting"},next:{title:"Error Handling",permalink:"/catlang/docs/error-handling"}},u={},p=[],d={toc:p};function f(e){var n=e.components,r=(0,o.Z)(e,i);return(0,a.kt)("wrapper",(0,t.Z)({},d,r,{components:n,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"Generics look similar to generics in other languages:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre"},"const contains = <T>(arr: []T, value: T) -> {\n  for (x in arr) {\n    if (x == value) {\n      return true;\n    }\n  }\n  return false;\n}\n")),(0,a.kt)("p",null,"When generic functions are called, the generic type can be inferred:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre"},"const arr = []int { 1, 2, 3, 4, 5 };\nconst result = contains(arr, 4);\n")),(0,a.kt)("p",null,"However, sometimes the generic type cannot be inferred and must be made explicit:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre"},"const arrayFactory = <T>(count: number) {\n  return new [count]T;\n}\nconst arr = arrayFactory(10); // ERROR!\nconst arr = arrayFactory<int>(10); // OK!\n")),(0,a.kt)("p",null,"Structs can also be made generic:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre"},"struct Node<K, V> {\n  key: K;\n  value: V;\n  leftChild?: Node<K, V>;\n  rightChild?: Node<K, V>;\n}\nstruct Tree<K, V> {\n  root: Node<K, V>;\n}\n\nTree<K, V>::get = (this, key: K) -> {\n  const getHelper = (node?: Node<K, V>) -> {\n    if (node) {\n      if (node.key == key) {\n        return node.value;\n      } else {\n        if (node.key < key) {\n          return getHelper(node.rightChild);\n        } else {\n          return getHelper(node.leftChild);\n        }\n      }\n    } else {\n      return null;\n    }\n  }\n  return getHelper(this.root);\n}\n\nconst t: Tree<int, string> = treeMaker();\nif (let v = t.get(12)) {\n  print(v);\n}\n")))}f.isMDXComponent=!0}}]);