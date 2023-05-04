use phf::phf_map;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileCategory {
    Generic,
    Folder,
    Archive,
    Audio,
    Code,
    Document,
    Image,
    Pdf,
    Presentation,
    Sheet,
    Text,
    Video,
}

static EXT_TO_FILE_CATEGORY: phf::Map<&'static str, FileCategory> = phf_map! {
    "3ds" => FileCategory::Image,
    "3g2" => FileCategory::Video,
    "3ga" => FileCategory::Video,
    "3gp" => FileCategory::Video,
    "3gp2" => FileCategory::Video,
    "3gpp" => FileCategory::Video,
    "3gpp2" => FileCategory::Video,
    "7z" => FileCategory::Archive,
    "aac" => FileCategory::Audio,
    "ac3" => FileCategory::Audio,
    "adb" => FileCategory::Text,
    "ads" => FileCategory::Text,
    "ag" => FileCategory::Image,
    "aif" => FileCategory::Audio,
    "aifc" => FileCategory::Audio,
    "aiff" => FileCategory::Audio,
    "aiffc" => FileCategory::Audio,
    "amr" => FileCategory::Audio,
    "amz" => FileCategory::Audio,
    "ape" => FileCategory::Audio,
    "arj" => FileCategory::Archive,
    "art" => FileCategory::Image,
    "arw" => FileCategory::Image,
    "asc" => FileCategory::Text,
    "asf" => FileCategory::Video,
    "asp" => FileCategory::Code,
    "aspx" => FileCategory::Code,
    "ass" => FileCategory::Text,
    "asx" => FileCategory::Video,
    "au" => FileCategory::Audio,
    "avf" => FileCategory::Video,
    "avi" => FileCategory::Video,
    "awb" => FileCategory::Audio,
    "axa" => FileCategory::Audio,
    "axv" => FileCategory::Video,
    "bat" => FileCategory::Text,
    "bdm" => FileCategory::Video,
    "bdmv" => FileCategory::Video,
    "bib" => FileCategory::Text,
    "bmp" => FileCategory::Image,
    "boo" => FileCategory::Text,
    "brf" => FileCategory::Text,
    "c" => FileCategory::Code,
    "c++" => FileCategory::Code,
    "cbl" => FileCategory::Text,
    "cc" => FileCategory::Text,
    "cdr" => FileCategory::Image,
    "cdt" => FileCategory::Image,
    "cgm" => FileCategory::Image,
    "class" => FileCategory::Code,
    "clpi" => FileCategory::Video,
    "cls" => FileCategory::Text,
    "cmake" => FileCategory::Text,
    "cob" => FileCategory::Text,
    "cpi" => FileCategory::Video,
    "cpp" => FileCategory::Code,
    "cpt" => FileCategory::Image,
    "cr2" => FileCategory::Image,
    "crw" => FileCategory::Image,
    "cs" => FileCategory::Text,
    "csd" => FileCategory::Audio,
    "csh" => FileCategory::Text,
    "css" => FileCategory::Code,
    "csv" => FileCategory::Text,
    "csvs" => FileCategory::Text,
    "cur" => FileCategory::Image,
    "cxx" => FileCategory::Text,
    "d" => FileCategory::Text,
    "dart" => FileCategory::Code,
    "dcl" => FileCategory::Text,
    "dcr" => FileCategory::Image,
    "dds" => FileCategory::Image,
    "deb" => FileCategory::Archive,
    "di" => FileCategory::Text,
    "dif" => FileCategory::Video,
    "diff" => FileCategory::Text,
    "divx" => FileCategory::Video,
    "djv" => FileCategory::Image,
    "djvu" => FileCategory::Image,
    "dl" => FileCategory::Video,
    "dng" => FileCategory::Image,
    "doc" => FileCategory::Document,
    "docx" => FileCategory::Document,
    "dot" => FileCategory::Document,
    "dotx" => FileCategory::Document,
    "dsl" => FileCategory::Text,
    "dts" => FileCategory::Audio,
    "dtshd" => FileCategory::Audio,
    "dtx" => FileCategory::Text,
    "dv" => FileCategory::Video,
    "dwg" => FileCategory::Image,
    "dxf" => FileCategory::Image,
    "e" => FileCategory::Text,
    "eif" => FileCategory::Text,
    "el" => FileCategory::Text,
    "emf" => FileCategory::Image,
    "eps" => FileCategory::Image,
    "epsf" => FileCategory::Image,
    "epsi" => FileCategory::Image,
    "erf" => FileCategory::Image,
    "erl" => FileCategory::Text,
    "etx" => FileCategory::Text,
    "exr" => FileCategory::Image,
    "f" => FileCategory::Text,
    "f4a" => FileCategory::Audio,
    "f4b" => FileCategory::Audio,
    "f4v" => FileCategory::Video,
    "f90" => FileCategory::Text,
    "f95" => FileCategory::Text,
    "fig" => FileCategory::Image,
    "fits" => FileCategory::Image,
    "flac" => FileCategory::Audio,
    "flc" => FileCategory::Video,
    "fli" => FileCategory::Video,
    "flv" => FileCategory::Video,
    "fo" => FileCategory::Text,
    "for" => FileCategory::Text,
    "fxm" => FileCategory::Video,
    "g3" => FileCategory::Image,
    "gcd" => FileCategory::Text,
    "gcrd" => FileCategory::Text,
    "gem" => FileCategory::Archive,
    "gif" => FileCategory::Image,
    "gl" => FileCategory::Video,
    "go" => FileCategory::Code,
    "gs" => FileCategory::Text,
    "gsm" => FileCategory::Audio,
    "gtar" => FileCategory::Archive,
    "gv" => FileCategory::Text,
    "gvp" => FileCategory::Text,
    "gz" => FileCategory::Archive,
    "h" => FileCategory::Code,
    "h++" => FileCategory::Text,
    "hh" => FileCategory::Text,
    "hp" => FileCategory::Text,
    "hpp" => FileCategory::Text,
    "hs" => FileCategory::Text,
    "htc" => FileCategory::Text,
    "htm" => FileCategory::Code,
    "html" => FileCategory::Code,
    "hxx" => FileCategory::Text,
    "icb" => FileCategory::Image,
    "icns" => FileCategory::Image,
    "ico" => FileCategory::Image,
    "ics" => FileCategory::Text,
    "icz" => FileCategory::Text,
    "idl" => FileCategory::Text,
    "ief" => FileCategory::Image,
    "iff" => FileCategory::Image,
    "ilbm" => FileCategory::Image,
    "ime" => FileCategory::Text,
    "imy" => FileCategory::Text,
    "ins" => FileCategory::Text,
    "iptables" => FileCategory::Text,
    "it" => FileCategory::Audio,
    "jad" => FileCategory::Text,
    "java" => FileCategory::Code,
    "jng" => FileCategory::Image,
    "jp2" => FileCategory::Image,
    "jpe" => FileCategory::Image,
    "jpeg" => FileCategory::Image,
    "jpf" => FileCategory::Image,
    "jpg" => FileCategory::Image,
    "jpg2" => FileCategory::Image,
    "jpm" => FileCategory::Image,
    "jpx" => FileCategory::Image,
    "js" => FileCategory::Code,
    "jsm" => FileCategory::Code,
    "json" => FileCategory::Code,
    "jsp" => FileCategory::Code,
    "jsx" => FileCategory::Code,
    "k25" => FileCategory::Image,
    "kar" => FileCategory::Audio,
    "kdc" => FileCategory::Image,
    "key" => FileCategory::Presentation,
    "ksh" => FileCategory::Text,
    "latex" => FileCategory::Text,
    "lbm" => FileCategory::Image,
    "ldif" => FileCategory::Text,
    "less" => FileCategory::Code,
    "lhs" => FileCategory::Text,
    "log" => FileCategory::Text,
    "lrv" => FileCategory::Video,
    "lsf" => FileCategory::Video,
    "lsx" => FileCategory::Video,
    "ltx" => FileCategory::Text,
    "lua" => FileCategory::Text,
    "lwo" => FileCategory::Image,
    "lwob" => FileCategory::Image,
    "lws" => FileCategory::Image,
    "ly" => FileCategory::Text,
    "lz" => FileCategory::Archive,
    "m" => FileCategory::Text,
    "m15" => FileCategory::Audio,
    "m1u" => FileCategory::Video,
    "m1v" => FileCategory::Video,
    "m2t" => FileCategory::Video,
    "m2ts" => FileCategory::Video,
    "m3u" => FileCategory::Audio,
    "m3u8" => FileCategory::Audio,
    "m4a" => FileCategory::Audio,
    "m4b" => FileCategory::Audio,
    "m4u" => FileCategory::Video,
    "m4v" => FileCategory::Video,
    "mak" => FileCategory::Text,
    "manifest" => FileCategory::Text,
    "markdown" => FileCategory::Text,
    "md" => FileCategory::Text,
    "mdi" => FileCategory::Image,
    "me" => FileCategory::Text,
    "med" => FileCategory::Audio,
    "mid" => FileCategory::Audio,
    "midi" => FileCategory::Audio,
    "minipsf" => FileCategory::Audio,
    "mk" => FileCategory::Text,
    "mk3d" => FileCategory::Video,
    "mka" => FileCategory::Audio,
    "mkd" => FileCategory::Text,
    "mkv" => FileCategory::Video,
    "ml" => FileCategory::Text,
    "mli" => FileCategory::Text,
    "mm" => FileCategory::Text,
    "mml" => FileCategory::Text,
    "mng" => FileCategory::Video,
    "mo" => FileCategory::Text,
    "mo3" => FileCategory::Audio,
    "moc" => FileCategory::Text,
    "mod" => FileCategory::Audio,
    "mof" => FileCategory::Text,
    "moov" => FileCategory::Video,
    "mov" => FileCategory::Video,
    "movie" => FileCategory::Video,
    "mp2" => FileCategory::Video,
    "mp3" => FileCategory::Audio,
    "mp4" => FileCategory::Video,
    "mpa" => FileCategory::Video,
    "mpc" => FileCategory::Audio,
    "mpe" => FileCategory::Video,
    "mpeg" => FileCategory::Video,
    "mpega" => FileCategory::Audio,
    "mpg" => FileCategory::Video,
    "mpga" => FileCategory::Audio,
    "mpl" => FileCategory::Video,
    "mpls" => FileCategory::Video,
    "mpp" => FileCategory::Audio,
    "mpv" => FileCategory::Video,
    "mrl" => FileCategory::Text,
    "mrml" => FileCategory::Text,
    "mrw" => FileCategory::Image,
    "ms" => FileCategory::Text,
    "msod" => FileCategory::Image,
    "mtm" => FileCategory::Audio,
    "mts" => FileCategory::Video,
    "mup" => FileCategory::Text,
    "mxu" => FileCategory::Video,
    "nef" => FileCategory::Image,
    "nfo" => FileCategory::Text,
    "not" => FileCategory::Text,
    "nsv" => FileCategory::Video,
    "ocl" => FileCategory::Text,
    "odp" => FileCategory::Presentation,
    "ods" => FileCategory::Sheet,
    "odt" => FileCategory::Document,
    "oga" => FileCategory::Audio,
    "ogg" => FileCategory::Audio,
    "ogm" => FileCategory::Video,
    "ogv" => FileCategory::Video,
    "ooc" => FileCategory::Text,
    "opml" => FileCategory::Text,
    "opus" => FileCategory::Audio,
    "ora" => FileCategory::Image,
    "orc" => FileCategory::Audio,
    "orf" => FileCategory::Image,
    "p" => FileCategory::Text,
    "pas" => FileCategory::Text,
    "pat" => FileCategory::Image,
    "patch" => FileCategory::Text,
    "pbm" => FileCategory::Image,
    "pcd" => FileCategory::Image,
    "pct" => FileCategory::Image,
    "pcx" => FileCategory::Image,
    "pdf" => FileCategory::Pdf,
    "pef" => FileCategory::Image,
    "pgm" => FileCategory::Image,
    "php" => FileCategory::Code,
    "pic" => FileCategory::Image,
    "pict" => FileCategory::Image,
    "pict1" => FileCategory::Image,
    "pict2" => FileCategory::Image,
    "pkg" => FileCategory::Archive,
    "pl" => FileCategory::Code,
    "pla" => FileCategory::Audio,
    "pls" => FileCategory::Audio,
    "pm" => FileCategory::Text,
    "png" => FileCategory::Image,
    "pnm" => FileCategory::Image,
    "pntg" => FileCategory::Image,
    "po" => FileCategory::Text,
    "pot" => FileCategory::Presentation,
    "potx" => FileCategory::Presentation,
    "ppa" => FileCategory::Presentation,
    "ppm" => FileCategory::Image,
    "pps" => FileCategory::Presentation,
    "ppsx" => FileCategory::Presentation,
    "ppt" => FileCategory::Presentation,
    "pptx" => FileCategory::Presentation,
    "ppz" => FileCategory::Presentation,
    "psd" => FileCategory::Image,
    "psf" => FileCategory::Audio,
    "psflib" => FileCategory::Audio,
    "psid" => FileCategory::Audio,
    "pwz" => FileCategory::Presentation,
    "py" => FileCategory::Code,
    "pyx" => FileCategory::Code,
    "qif" => FileCategory::Image,
    "qml" => FileCategory::Text,
    "qmlproject" => FileCategory::Text,
    "qmltypes" => FileCategory::Text,
    "qt" => FileCategory::Video,
    "qtif" => FileCategory::Image,
    "qtvr" => FileCategory::Video,
    "ra" => FileCategory::Audio,
    "raf" => FileCategory::Image,
    "ram" => FileCategory::Audio,
    "rar" => FileCategory::Archive,
    "ras" => FileCategory::Image,
    "raw" => FileCategory::Image,
    "rax" => FileCategory::Audio,
    "rb" => FileCategory::Code,
    "reg" => FileCategory::Text,
    "rej" => FileCategory::Text,
    "rgb" => FileCategory::Image,
    "rle" => FileCategory::Image,
    "rm" => FileCategory::Audio,
    "roff" => FileCategory::Text,
    "rp" => FileCategory::Image,
    "rpm" => FileCategory::Archive,
    "rs" => FileCategory::Code,
    "rss" => FileCategory::Code,
    "rt" => FileCategory::Text,
    "rtf" => FileCategory::Text,
    "rtx" => FileCategory::Text,
    "rv" => FileCategory::Video,
    "rvx" => FileCategory::Video,
    "rw2" => FileCategory::Image,
    "s3m" => FileCategory::Audio,
    "sass" => FileCategory::Code,
    "scala" => FileCategory::Text,
    "scm" => FileCategory::Text,
    "sco" => FileCategory::Audio,
    "scss" => FileCategory::Code,
    "sct" => FileCategory::Text,
    "sd2" => FileCategory::Audio,
    "sfv" => FileCategory::Text,
    "sgi" => FileCategory::Image,
    "sgm" => FileCategory::Text,
    "sgml" => FileCategory::Text,
    "sh" => FileCategory::Code,
    "shtml" => FileCategory::Code,
    "sid" => FileCategory::Audio,
    "sk" => FileCategory::Image,
    "sk1" => FileCategory::Image,
    "sldx" => FileCategory::Presentation,
    "slk" => FileCategory::Text,
    "snd" => FileCategory::Audio,
    "spec" => FileCategory::Text,
    "spx" => FileCategory::Audio,
    "sql" => FileCategory::Code,
    "sr2" => FileCategory::Image,
    "srf" => FileCategory::Image,
    "srt" => FileCategory::Text,
    "ss" => FileCategory::Text,
    "ssa" => FileCategory::Text,
    "stm" => FileCategory::Audio,
    "sty" => FileCategory::Text,
    "sub" => FileCategory::Text,
    "sun" => FileCategory::Image,
    "sv" => FileCategory::Text,
    "svg" => FileCategory::Image,
    "svgz" => FileCategory::Image,
    "svh" => FileCategory::Text,
    "swift" => FileCategory::Code,
    "sylk" => FileCategory::Text,
    "t" => FileCategory::Text,
    "t2t" => FileCategory::Text,
    "tar" => FileCategory::Archive,
    "tcl" => FileCategory::Text,
    "tex" => FileCategory::Text,
    "texi" => FileCategory::Text,
    "texinfo" => FileCategory::Text,
    "text" => FileCategory::Text,
    "tga" => FileCategory::Image,
    "tif" => FileCategory::Image,
    "tiff" => FileCategory::Image,
    "tk" => FileCategory::Text,
    "tm" => FileCategory::Text,
    "toml" => FileCategory::Code,
    "tpic" => FileCategory::Image,
    "tr" => FileCategory::Text,
    "ts" => FileCategory::Code,
    "tsv" => FileCategory::Text,
    "tsx" => FileCategory::Code,
    "tta" => FileCategory::Audio,
    "ttl" => FileCategory::Text,
    "txt" => FileCategory::Text,
    "uil" => FileCategory::Text,
    "uls" => FileCategory::Text,
    "ult" => FileCategory::Audio,
    "uni" => FileCategory::Audio,
    "uue" => FileCategory::Text,
    "v" => FileCategory::Text,
    "vala" => FileCategory::Text,
    "vapi" => FileCategory::Text,
    "vb" => FileCategory::Code,
    "vcard" => FileCategory::Text,
    "vcf" => FileCategory::Text,
    "vcs" => FileCategory::Text,
    "vct" => FileCategory::Text,
    "vda" => FileCategory::Image,
    "vhd" => FileCategory::Text,
    "vhdl" => FileCategory::Text,
    "viv" => FileCategory::Video,
    "vivo" => FileCategory::Video,
    "vlc" => FileCategory::Audio,
    "vob" => FileCategory::Video,
    "voc" => FileCategory::Audio,
    "vst" => FileCategory::Image,
    "vtt" => FileCategory::Text,
    "wav" => FileCategory::Audio,
    "wax" => FileCategory::Audio,
    "wbmp" => FileCategory::Image,
    "webm" => FileCategory::Video,
    "webp" => FileCategory::Image,
    "wiz" => FileCategory::Document,
    "wm" => FileCategory::Video,
    "wma" => FileCategory::Audio,
    "wmf" => FileCategory::Image,
    "wml" => FileCategory::Text,
    "wmls" => FileCategory::Text,
    "wmv" => FileCategory::Video,
    "wmx" => FileCategory::Video,
    "wsc" => FileCategory::Text,
    "wsgi" => FileCategory::Text,
    "wv" => FileCategory::Audio,
    "wvc" => FileCategory::Audio,
    "wvp" => FileCategory::Audio,
    "wvx" => FileCategory::Video,
    "x3f" => FileCategory::Image,
    "xbm" => FileCategory::Image,
    "xcf" => FileCategory::Image,
    "xi" => FileCategory::Audio,
    "xla" => FileCategory::Sheet,
    "xlam" => FileCategory::Sheet,
    "xlb" => FileCategory::Sheet,
    "xlc" => FileCategory::Sheet,
    "xld" => FileCategory::Sheet,
    "xll" => FileCategory::Sheet,
    "xlm" => FileCategory::Sheet,
    "xls" => FileCategory::Sheet,
    "xlsb" => FileCategory::Sheet,
    "xlsm" => FileCategory::Sheet,
    "xlsx" => FileCategory::Sheet,
    "xlt" => FileCategory::Sheet,
    "xltx" => FileCategory::Sheet,
    "xlw" => FileCategory::Sheet,
    "xm" => FileCategory::Audio,
    "xmf" => FileCategory::Audio,
    "xmi" => FileCategory::Text,
    "xml" => FileCategory::Code,
    "xpm" => FileCategory::Image,
    "xslfo" => FileCategory::Text,
    "xul" => FileCategory::Text,
    "xwd" => FileCategory::Image,
    "yaml" => FileCategory::Code,
    "z" => FileCategory::Archive,
    "zip" => FileCategory::Archive,
};

pub fn ext_to_file_category(ext_lower: &str) -> Option<FileCategory> {
    EXT_TO_FILE_CATEGORY.get(ext_lower).cloned()
}
