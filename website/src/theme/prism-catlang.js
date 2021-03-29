(function (Prism) {
  var multilineComment = /\/\*(?:[^*/]|\*(?!\/)|\/(?!\*)|<self>)*\*\//.source;
  for (var i = 0; i < 2; i++) {
    // support 4 levels of nested comments
    multilineComment = multilineComment.replace(/<self>/g, function () { return multilineComment; });
  }
  multilineComment = multilineComment.replace(/<self>/g, function () { return /[^\s\S]/.source; });

  Prism.languages.catlang = {
    'comment': [
      {
        pattern: RegExp(/(^|[^\\])/.source + multilineComment),
        lookbehind: true,
        greedy: true
      },
      {
				pattern: /(^|[^\\:])\/\/.*/,
				lookbehind: true,
				greedy: true
      }
    ],
    'string': {
      pattern: /b?"(?:\\[\s\S]|[^\\"])*"|b?r(#*)"(?:[^"]|"(?!\1))*"\1/,
			greedy: true
    },
    'char': {
			pattern: /b?'(?:\\(?:x[0-7][\da-fA-F]|u\{(?:[\da-fA-F]_*){1,6}\}|.)|[^\\\r\n\t'])'/,
			greedy: true,
			alias: 'string'
    },
    'attribute': {
			pattern: /#!?\[(?:[^\[\]"]|"(?:\\[\s\S]|[^\\"])*")*\]/,
			greedy: true,
			alias: 'attr-name',
			inside: {
				'string': null // see below
			}
    },

    'variable': /\$\w+/,

    'function-definition': {
			pattern: /(\bfunction\s+)\w+/,
			lookbehind: true,
			alias: 'function'
    },
    'type-definition': {
			pattern: /(\b(?:enum|struct|union)\s+)\w+/,
			lookbehind: true,
			alias: 'class-name'
    },
    'keyword': [
      /\b(?:any|let|const|new|delete|typeof|is|as|in|function|return|struct|type|enum|owned|import|export|from|for|while|do|loop|if|else|break|continue|null|this|volatile|unreachable|namespace|using)\b/,
      // Primitives
      /\b(?:[us](?:8|16|32|64)|char|(?:c_|c_u)(?:short|int|long|longlong)|c_longdouble|bool|f(?:32|64)|float|double|null|c_void|type)\b/
    ],

    'function': /\b[a-z_]\w*(?=\s*(?:::\s*<|\())/,
    'constant': /\b[A-Z_][A-Z_\d]+\b/,

    'class-name': {
      pattern: /(\b(?:enum|struct)\s+(?:__attribute__\s*\(\([\s\S]*?\)\)\s*)?)\w+|\b[a-z]\w*_t\b/,
      lookbehind: true
    },

    'number': /(?:\b0x(?:[\da-f]+(?:\.[\da-f]*)?|\.[\da-f]+)(?:p[+-]?\d+)?|(?:\b\d+(?:\.\d*)?|\B\.\d+)(?:e[+-]?\d+)?)[ful]{0,4}/i,
    'boolean': /\b(?:false|true)\b/,
    'punctuation': /->|\.\.=|\.{1,3}|::|[{}[\];(),:]/,
    'operator': /[-+*\/%!^]=?|=[=>]?|&[&=]?|\|[|=]?|<<?=?|>>?=?|[@?]/
  };
}(Prism));