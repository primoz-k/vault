function fileNameExtension(name: string): string | undefined {
  const parts = name.split('.');
  return parts.length > 1 ? parts[parts.length - 1].toLowerCase() : undefined;
}

export const monacoLanguages = new Map<string, string | undefined>([
  ['abap', 'abap'],
  ['cls', 'apex'],
  ['azcli', 'azcli'],
  ['bat', 'bat'],
  ['cmd', 'bat'],
  ['mligo', 'cameligo'],
  ['clj', 'clojure'],
  ['cljs', 'clojure'],
  ['cljc', 'clojure'],
  ['edn', 'clojure'],
  ['coffee', 'coffeescript'],
  ['cakefile', 'coffeescript'],
  ['c', 'c'],
  ['h', 'c'],
  ['cpp', 'cpp'],
  ['cc', 'cpp'],
  ['cxx', 'cpp'],
  ['hpp', 'cpp'],
  ['hh', 'cpp'],
  ['hxx', 'cpp'],
  ['cs', 'csharp'],
  ['csx', 'csharp'],
  ['cake', 'csharp'],
  ['css', 'css'],
  ['dockerfile', 'dockerfile'],
  ['fs', 'fsharp'],
  ['fsi', 'fsharp'],
  ['ml', 'fsharp'],
  ['mli', 'fsharp'],
  ['fsx', 'fsharp'],
  ['fsscript', 'fsharp'],
  ['go', 'go'],
  ['graphql', 'graphql'],
  ['gql', 'graphql'],
  ['handlebars', 'handlebars'],
  ['hbs', 'handlebars'],
  ['html', 'html'],
  ['htm', 'html'],
  ['shtml', 'html'],
  ['xhtml', 'html'],
  ['mdoc', 'html'],
  ['jsp', 'html'],
  ['asp', 'html'],
  ['aspx', 'html'],
  ['jshtm', 'html'],
  ['ini', 'ini'],
  ['properties', 'ini'],
  ['gitconfig', 'ini'],
  ['java', 'java'],
  ['jav', 'java'],
  ['js', 'javascript'],
  ['es6', 'javascript'],
  ['jsx', 'javascript'],
  ['json', 'json'],
  ['kt', 'kotlin'],
  ['less', 'less'],
  ['lua', 'lua'],
  ['md', 'markdown'],
  ['markdown', 'markdown'],
  ['mdown', 'markdown'],
  ['mkdn', 'markdown'],
  ['mkd', 'markdown'],
  ['mdwn', 'markdown'],
  ['mdtxt', 'markdown'],
  ['mdtext', 'markdown'],
  ['lr', 'markdown'],
  ['s', 'mips'],
  ['dax', 'msdax'],
  ['msdax', 'msdax'],
  ['m', 'objective-c'],
  ['pas', 'pascal'],
  ['p', 'pascal'],
  ['pp', 'pascal'],
  ['ligo', 'pascaligo'],
  ['pl', 'perl'],
  ['pm', 'perl'],
  ['php', 'php'],
  ['php4', 'php'],
  ['php5', 'php'],
  ['phtml', 'php'],
  ['ctp', 'php'],
  ['dats', 'postiats'],
  ['sats', 'postiats'],
  ['hats', 'postiats'],
  ['pq', 'powerquery'],
  ['pqm', 'powerquery'],
  ['ps1', 'powershell'],
  ['psm1', 'powershell'],
  ['psd1', 'powershell'],
  ['jade', 'pug'],
  ['pug', 'pug'],
  ['py', 'python'],
  ['rpy', 'python'],
  ['pyw', 'python'],
  ['cpy', 'python'],
  ['gyp', 'python'],
  ['gypi', 'python'],
  ['r', 'r'],
  ['rhistory', 'r'],
  ['rprofile', 'r'],
  ['rt', 'r'],
  ['cshtml', 'razor'],
  ['redis', 'redis'],
  ['rst', 'restructuredtext'],
  ['rb', 'ruby'],
  ['rbx', 'ruby'],
  ['rjs', 'ruby'],
  ['gemspec', 'ruby'],
  ['rake', 'ruby'],
  ['pp', 'ruby'],
  ['rs', 'rust'],
  ['rlib', 'rust'],
  ['sb', 'sb'],
  ['scm', 'scheme'],
  ['ss', 'scheme'],
  ['sch', 'scheme'],
  ['rkt', 'scheme'],
  ['scss', 'scss'],
  ['sh', 'shell'],
  ['bash', 'shell'],
  ['sol', 'sol'],
  ['aes', 'aes'],
  ['sql', 'sql'],
  ['pgsql', 'pgsql'],
  ['st', 'st'],
  ['iecst', 'st'],
  ['iecplc', 'st'],
  ['lc3lib', 'st'],
  ['swift', 'swift'],
  ['tcl', 'tcl'],
  ['twig', 'twig'],
  ['ts', 'typescript'],
  ['tsx', 'typescript'],
  ['vb', 'vb'],
  ['xml', 'xml'],
  ['dtd', 'xml'],
  ['ascx', 'xml'],
  ['csproj', 'xml'],
  ['config', 'xml'],
  ['wxi', 'xml'],
  ['wxl', 'xml'],
  ['wxs', 'xml'],
  ['xaml', 'xml'],
  ['svg', 'xml'],
  ['svgz', 'xml'],
  ['opf', 'xml'],
  ['xsl', 'xml'],
  ['atom', 'xml'],
  ['mathml', 'xml'],
  ['mml', 'xml'],
  ['rdf', 'xml'],
  ['rss', 'xml'],
  ['wsdl', 'xml'],
  ['xbl', 'xml'],
  ['xslt', 'xml'],
  ['xul', 'xml'],
  ['yaml', 'yaml'],
  ['yml', 'yaml'],
  ['txt', 'plaintext'],
  ['bib', 'plaintext'],
  ['latex', 'plaintext'],
  ['ltx', 'plaintext'],
  ['tex', 'plaintext'],
  ['diff', 'plaintext'],
  ['frag', 'plaintext'],
  ['glsl', 'plaintext'],
  ['vert', 'plaintext'],
  ['groovy', 'plaintext'],
  ['hx', 'plaintext'],
  ['lp', 'plaintext'],
  ['mli', 'plaintext'],
  ['ml', 'plaintext'],
  ['patch', 'plaintext'],
  ['scad', 'plaintext'],
  ['scala', 'plaintext'],
  ['textile', 'plaintext'],
  ['xq', 'plaintext'],
]);

export const monacoLanguageForFileName = (
  fileName: string,
): string | undefined => {
  const ext = fileNameExtension(fileName);

  return ext !== undefined ? monacoLanguages.get(ext) : undefined;
};
