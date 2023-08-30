use phf::phf_map;

static EXT_TO_CONTENT_TYPE: phf::Map<&'static str, &'static str> = phf_map! {
    "3ds" => "image/x-3ds",
    "3g2" => "video/3gpp2",
    "3gp" => "video/3gpp",
    "3gpp" => "video/3gpp",
    "7z" => "application/x-7z-compressed",
    "aac" => "audio/x-aac",
    "aif" => "audio/x-aiff",
    "aifc" => "audio/x-aiff",
    "aiff" => "audio/x-aiff",
    "asc" => "application/pgp-signature",
    "asf" => "video/x-ms-asf",
    "asx" => "video/x-ms-asf",
    "au" => "audio/basic",
    "avi" => "video/x-msvideo",
    "bat" => "text/plain",
    "bdm" => "application/vnd.syncml.dm+wbxml",
    "bmp" => "image/x-ms-bmp",
    "c" => "text/plain",
    "cc" => "text/x-c",
    "cgm" => "image/cgm",
    "class" => "application/java-vm",
    "cpp" => "text/x-c",
    "cpt" => "application/mac-compactpro",
    "csh" => "application/x-csh",
    "css" => "text/css",
    "csv" => "text/csv",
    "cxx" => "text/x-c",
    "dart" => "application/vnd.dart",
    "dcr" => "application/x-director",
    "dif" => "video/x-dv",
    "djv" => "image/vnd.djvu",
    "djvu" => "image/vnd.djvu",
    "doc" => "application/msword",
    "docm" => "application/vnd.ms-word.document.macroEnabled.12",
    "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "dot" => "application/msword",
    "dotx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
    "dts" => "audio/vnd.dts",
    "dtshd" => "audio/vnd.dts.hd",
    "dv" => "video/x-dv",
    "dwg" => "image/vnd.dwg",
    "dxf" => "image/vnd.dxf",
    "emf" => "application/x-msmetafile",
    "eps" => "application/postscript",
    "etx" => "text/x-setext",
    "f" => "text/x-fortran",
    "f4v" => "video/x-f4v",
    "f90" => "text/x-fortran",
    "fig" => "application/x-xfig",
    "flac" => "audio/x-flac",
    "fli" => "video/x-fli",
    "flv" => "video/x-flv",
    "for" => "text/x-fortran",
    "g3" => "image/g3fax",
    "gif" => "image/gif",
    "gtar" => "application/x-gtar",
    "gv" => "text/vnd.graphviz",
    "gz" => "application/gzip",
    "h" => "text/plain",
    "heic" => "image/heic",
    "hh" => "text/x-c",
    "htc" => "text/x-component",
    "htm" => "text/html",
    "html" => "text/html",
    "ico" => "image/x-icon",
    "ics" => "text/calendar",
    "ief" => "image/ief",
    "jad" => "text/vnd.sun.j2me.app-descriptor",
    "java" => "text/x-java-source",
    "jng" => "image/x-jng",
    "jp2" => "image/jp2",
    "jpe" => "image/jpeg",
    "jpeg" => "image/jpeg",
    "jpg" => "image/jpeg",
    "jpm" => "video/jpm",
    "js" => "application/javascript",
    "json" => "application/json",
    "kar" => "audio/midi",
    "ksh" => "text/plain",
    "latex" => "application/x-latex",
    "log" => "text/plain",
    "m1v" => "video/mpeg",
    "m3u" => "audio/x-mpegurl",
    "m3u8" => "application/x-mpegurl",
    "m4a" => "audio/x-m4a",
    "m4u" => "video/vnd.mpegurl",
    "m4v" => "video/x-m4v",
    "manifest" => "text/cache-manifest",
    "mdi" => "image/vnd.ms-modi",
    "me" => "application/x-troff-me",
    "mid" => "audio/midi",
    "midi" => "audio/midi",
    "mk3d" => "video/x-matroska",
    "mka" => "audio/x-matroska",
    "mkv" => "video/x-matroska",
    "mml" => "text/mathml",
    "mng" => "video/x-mng",
    "mov" => "video/quicktime",
    "movie" => "video/x-sgi-movie",
    "mp2" => "audio/mpeg",
    "mp3" => "audio/mpeg",
    "mp4" => "video/mp4",
    "mpa" => "video/mpeg",
    "mpc" => "application/vnd.mophun.certificate",
    "mpe" => "video/mpeg",
    "mpeg" => "video/mpeg",
    "mpg" => "video/mpeg",
    "mpga" => "audio/mpeg",
    "mpp" => "application/vnd.ms-project",
    "ms" => "application/x-troff-ms",
    "mts" => "model/vnd.mts",
    "mxu" => "video/vnd.mpegurl",
    "nfo" => "text/x-nfo",
    "odp" => "application/vnd.oasis.opendocument.presentation",
    "ods" => "application/vnd.oasis.opendocument.spreadsheet",
    "odt" => "application/vnd.oasis.opendocument.text",
    "oga" => "audio/ogg",
    "ogg" => "audio/ogg",
    "ogv" => "video/ogg",
    "opml" => "text/x-opml",
    "p" => "text/x-pascal",
    "pas" => "text/x-pascal",
    "pbm" => "image/x-portable-bitmap",
    "pct" => "image/pict",
    "pcx" => "image/x-pcx",
    "pdf" => "application/pdf",
    "pgm" => "image/x-portable-graymap",
    "pic" => "image/pict",
    "pict" => "image/pict",
    "pl" => "text/plain",
    "pls" => "application/pls+xml",
    "pm" => "application/x-perl",
    "png" => "image/png",
    "pnm" => "image/x-portable-anymap",
    "pntg" => "image/x-macpaint",
    "pot" => "application/vnd.ms-powerpoint",
    "potm" => "application/vnd.ms-powerpoint.template.macroEnabled.12",
    "potx" => "application/vnd.openxmlformats-officedocument.presentationml.template",
    "ppa" => "application/vnd.ms-powerpoint",
    "ppm" => "image/x-portable-pixmap",
    "pps" => "application/vnd.ms-powerpoint",
    "ppsm" => "application/vnd.ms-powerpoint.slideshow.macroEnabled.12",
    "ppsx" => "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
    "ppt" => "application/vnd.ms-powerpoint",
    "pptm" => "application/vnd.ms-powerpoint.presentation.macroEnabled.12",
    "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "psd" => "image/vnd.adobe.photoshop",
    "psf" => "application/x-font-linux-psf",
    "pwz" => "application/vnd.ms-powerpoint",
    "py" => "text/x-python",
    "qt" => "video/quicktime",
    "qtif" => "image/x-quicktime",
    "ra" => "audio/x-pn-realaudio",
    "ram" => "application/x-pn-realaudio",
    "rar" => "application/x-rar-compressed",
    "ras" => "image/x-cmu-raster",
    "rgb" => "image/x-rgb",
    "rm" => "application/vnd.rn-realmedia",
    "roff" => "application/x-troff",
    "rpm" => "application/x-redhat-package-manager",
    "rs" => "application/rls-services+xml",
    "rss" => "application/rss+xml",
    "rtf" => "application/rtf",
    "rtx" => "text/richtext",
    "s3m" => "audio/s3m",
    "scm" => "application/vnd.lotus-screencam",
    "sfv" => "text/x-sfv",
    "sgi" => "image/sgi",
    "sgm" => "text/x-sgml",
    "sgml" => "text/x-sgml",
    "sh" => "application/x-sh",
    "shtml" => "text/html",
    "sid" => "image/x-mrsid-image",
    "sldx" => "application/vnd.openxmlformats-officedocument.presentationml.slide",
    "snd" => "audio/basic",
    "spx" => "audio/ogg",
    "sql" => "application/x-sql",
    "srt" => "application/x-subrip",
    "sub" => "text/vnd.dvb.subtitle",
    "svg" => "image/svg+xml",
    "svgz" => "image/svg+xml",
    "t" => "application/x-troff",
    "tar" => "application/x-tar",
    "tcl" => "application/x-tcl",
    "tex" => "application/x-tex",
    "texi" => "application/x-texinfo",
    "texinfo" => "application/x-texinfo",
    "text" => "text/plain",
    "tga" => "image/x-tga",
    "tif" => "image/tiff",
    "tiff" => "image/tiff",
    "tk" => "application/x-tcl",
    "tr" => "application/x-troff",
    "ts" => "video/mp2t",
    "tsv" => "text/tab-separated-values",
    "ttl" => "text/turtle",
    "txt" => "text/plain",
    "vcard" => "text/vcard",
    "vcf" => "text/x-vcard",
    "vcs" => "text/x-vcalendar",
    "viv" => "video/vnd.vivo",
    "vob" => "video/x-ms-vob",
    "vst" => "application/vnd.visio",
    "wav" => "audio/x-wav",
    "wax" => "audio/x-ms-wax",
    "wbmp" => "image/vnd.wap.wbmp",
    "webm" => "video/webm",
    "webp" => "image/webp",
    "wiz" => "application/msword",
    "wm" => "video/x-ms-wm",
    "wma" => "audio/x-ms-wma",
    "wmf" => "application/x-msmetafile",
    "wml" => "text/vnd.wap.wml",
    "wmls" => "text/vnd.wap.wmlscript",
    "wmv" => "video/x-ms-wmv",
    "wmx" => "video/x-ms-wmx",
    "wvx" => "video/x-ms-wvx",
    "xbm" => "image/x-xbitmap",
    "xla" => "application/vnd.ms-excel",
    "xlam" => "application/vnd.ms-excel.addin.macroenabled.12",
    "xlb" => "application/vnd.ms-excel",
    "xlc" => "application/vnd.ms-excel",
    "xlm" => "application/vnd.ms-excel",
    "xls" => "application/vnd.ms-excel",
    "xlsb" => "application/vnd.ms-excel.sheet.binary.macroenabled.12",
    "xlsm" => "application/vnd.ms-excel.sheet.macroenabled.12",
    "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "xlt" => "application/vnd.ms-excel",
    "xltx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
    "xlw" => "application/vnd.ms-excel",
    "xm" => "audio/xm",
    "xml" => "text/xml",
    "xpm" => "image/x-xpixmap",
    "xul" => "text/xul",
    "xwd" => "image/x-xwindowdump",
    "yaml" => "application/yaml",
    "zip" => "application/zip",
};

pub fn ext_to_content_type(ext_lower: &str) -> Option<&str> {
    EXT_TO_CONTENT_TYPE.get(ext_lower).map(|x| *x)
}
