use std::path::Path;

fn slice_split_once<T: Eq>(slice: &[T], divisor: T) -> Option<(&[T], &[T])> {
    slice.iter().position(|x| *x == divisor).map(|i| (&slice[..i], &slice[i + 1 ..]))
}

/// Returns "text/plain" if the file extension is unrecognised
pub fn path_to_mime_type(path: impl AsRef<Path>) -> &'static str {
    let path = path.as_ref().as_os_str().as_encoded_bytes();
    let Some(ext) = slice_split_once(path, b'.').map(|(_base, ext)| ext) else {
        return "text/plain";
    };
    match ext {
        b"323" => "text/h323",
        b"aaf" => "application/octet-stream",
        b"aca" => "application/octet-stream",
        b"accdb" => "application/msaccess",
        b"accde" => "application/msaccess",
        b"accdt" => "application/msaccess",
        b"acx" => "application/internet-property-stream",
        b"afm" => "application/octet-stream",
        b"ai" => "application/postscript",
        b"aif" => "audio/x-aiff",
        b"aifc" => "audio/aiff",
        b"aiff" => "audio/aiff",
        b"application" => "application/x-ms-application",
        b"art" => "image/x-jg",
        b"asd" => "application/octet-stream",
        b"asf" => "video/x-ms-asf",
        b"asi" => "application/octet-stream",
        b"asm" => "text/plain",
        b"asr" => "video/x-ms-asf",
        b"asx" => "video/x-ms-asf",
        b"atom" => "application/atom+xml",
        b"au" => "audio/basic",
        b"avi" => "video/x-msvideo",
        b"axs" => "application/olescript",
        b"bas" => "text/plain",
        b"bcpio" => "application/x-bcpio",
        b"bin" => "application/octet-stream",
        b"bmp" => "image/bmp",
        b"c" => "text/plain",
        b"cab" => "application/octet-stream",
        b"calx" => "application/vnd.ms-office.calx",
        b"cat" => "application/vnd.ms-pki.seccat",
        b"cdf" => "application/x-cdf",
        b"chm" => "application/octet-stream",
        b"class" => "application/x-java-applet",
        b"clp" => "application/x-msclip",
        b"cmx" => "image/x-cmx",
        b"cnf" => "text/plain",
        b"cod" => "image/cis-cod",
        b"cpio" => "application/x-cpio",
        b"cpp" => "text/plain",
        b"crd" => "application/x-mscardfile",
        b"crl" => "application/pkix-crl",
        b"crt" => "application/x-x509-ca-cert",
        b"csh" => "application/x-csh",
        b"css" => "text/css",
        b"csv" => "application/octet-stream",
        b"cur" => "application/octet-stream",
        b"dcr" => "application/x-director",
        b"deploy" => "application/octet-stream",
        b"der" => "application/x-x509-ca-cert",
        b"dib" => "image/bmp",
        b"dir" => "application/x-director",
        b"disco" => "text/xml",
        b"dll" => "application/x-msdownload",
        b"dll.config" => "text/xml",
        b"dlm" => "text/dlm",
        b"doc" => "application/msword",
        b"docm" => "application/vnd.ms-word.document.macroEnabled.12",
        b"docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        b"dot" => "application/msword",
        b"dotm" => "application/vnd.ms-word.template.macroEnabled.12",
        b"dotx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
        b"dsp" => "application/octet-stream",
        b"dtd" => "text/xml",
        b"dvi" => "application/x-dvi",
        b"dwf" => "drawing/x-dwf",
        b"dwp" => "application/octet-stream",
        b"dxr" => "application/x-director",
        b"eml" => "message/rfc822",
        b"emz" => "application/octet-stream",
        b"eot" => "application/octet-stream",
        b"eps" => "application/postscript",
        b"etx" => "text/x-setext",
        b"evy" => "application/envoy",
        b"exe" => "application/octet-stream",
        b"exe.config" => "text/xml",
        b"fdf" => "application/vnd.fdf",
        b"fif" => "application/fractals",
        b"fla" => "application/octet-stream",
        b"flr" => "x-world/x-vrml",
        b"flv" => "video/x-flv",
        b"gif" => "image/gif",
        b"gtar" => "application/x-gtar",
        b"gz" => "application/x-gzip",
        b"h" => "text/plain",
        b"hdf" => "application/x-hdf",
        b"hdml" => "text/x-hdml",
        b"hhc" => "application/x-oleobject",
        b"hhk" => "application/octet-stream",
        b"hhp" => "application/octet-stream",
        b"hlp" => "application/winhlp",
        b"hqx" => "application/mac-binhex40",
        b"hta" => "application/hta",
        b"htc" => "text/x-component",
        b"htm" => "text/html",
        b"html" => "text/html",
        b"htt" => "text/webviewhtml",
        b"hxt" => "text/html",
        b"ico" => "image/x-icon",
        b"ics" => "application/octet-stream",
        b"ief" => "image/ief",
        b"iii" => "application/x-iphone",
        b"inf" => "application/octet-stream",
        b"ins" => "application/x-internet-signup",
        b"isp" => "application/x-internet-signup",
        b"IVF" => "video/x-ivf",
        b"jar" => "application/java-archive",
        b"java" => "application/octet-stream",
        b"jck" => "application/liquidmotion",
        b"jcz" => "application/liquidmotion",
        b"jfif" => "image/pjpeg",
        b"jpb" => "application/octet-stream",
        b"jpe" => "image/jpeg",
        b"jpeg" => "image/jpeg",
        b"jpg" => "image/jpeg",
        b"js" => "application/x-javascript",
        b"jsx" => "text/jscript",
        b"latex" => "application/x-latex",
        b"lit" => "application/x-ms-reader",
        b"lpk" => "application/octet-stream",
        b"lsf" => "video/x-la-asf",
        b"lsx" => "video/x-la-asf",
        b"lzh" => "application/octet-stream",
        b"m13" => "application/x-msmediaview",
        b"m14" => "application/x-msmediaview",
        b"m1v" => "video/mpeg",
        b"m3u" => "audio/x-mpegurl",
        b"man" => "application/x-troff-man",
        b"manifest" => "application/x-ms-manifest",
        b"map" => "text/plain",
        b"mdb" => "application/x-msaccess",
        b"mdp" => "application/octet-stream",
        b"me" => "application/x-troff-me",
        b"mht" => "message/rfc822",
        b"mhtml" => "message/rfc822",
        b"mid" => "audio/mid",
        b"midi" => "audio/mid",
        b"mix" => "application/octet-stream",
        b"mmf" => "application/x-smaf",
        b"mno" => "text/xml",
        b"mny" => "application/x-msmoney",
        b"mov" => "video/quicktime",
        b"movie" => "video/x-sgi-movie",
        b"mp2" => "video/mpeg",
        b"mp3" => "audio/mpeg",
        b"mpa" => "video/mpeg",
        b"mpe" => "video/mpeg",
        b"mpeg" => "video/mpeg",
        b"mpg" => "video/mpeg",
        b"mpp" => "application/vnd.ms-project",
        b"mpv2" => "video/mpeg",
        b"ms" => "application/x-troff-ms",
        b"msi" => "application/octet-stream",
        b"mso" => "application/octet-stream",
        b"mvb" => "application/x-msmediaview",
        b"mvc" => "application/x-miva-compiled",
        b"nc" => "application/x-netcdf",
        b"nsc" => "video/x-ms-asf",
        b"nws" => "message/rfc822",
        b"ocx" => "application/octet-stream",
        b"oda" => "application/oda",
        b"odc" => "text/x-ms-odc",
        b"ods" => "application/oleobject",
        b"one" => "application/onenote",
        b"onea" => "application/onenote",
        b"onetoc" => "application/onenote",
        b"onetoc2" => "application/onenote",
        b"onetmp" => "application/onenote",
        b"onepkg" => "application/onenote",
        b"osdx" => "application/opensearchdescription+xml",
        b"p10" => "application/pkcs10",
        b"p12" => "application/x-pkcs12",
        b"p7b" => "application/x-pkcs7-certificates",
        b"p7c" => "application/pkcs7-mime",
        b"p7m" => "application/pkcs7-mime",
        b"p7r" => "application/x-pkcs7-certreqresp",
        b"p7s" => "application/pkcs7-signature",
        b"pbm" => "image/x-portable-bitmap",
        b"pcx" => "application/octet-stream",
        b"pcz" => "application/octet-stream",
        b"pdf" => "application/pdf",
        b"pfb" => "application/octet-stream",
        b"pfm" => "application/octet-stream",
        b"pfx" => "application/x-pkcs12",
        b"pgm" => "image/x-portable-graymap",
        b"pko" => "application/vnd.ms-pki.pko",
        b"pma" => "application/x-perfmon",
        b"pmc" => "application/x-perfmon",
        b"pml" => "application/x-perfmon",
        b"pmr" => "application/x-perfmon",
        b"pmw" => "application/x-perfmon",
        b"png" => "image/png",
        b"pnm" => "image/x-portable-anymap",
        b"pnz" => "image/png",
        b"pot" => "application/vnd.ms-powerpoint",
        b"potm" => "application/vnd.ms-powerpoint.template.macroEnabled.12",
        b"potx" => "application/vnd.openxmlformats-officedocument.presentationml.template",
        b"ppam" => "application/vnd.ms-powerpoint.addin.macroEnabled.12",
        b"ppm" => "image/x-portable-pixmap",
        b"pps" => "application/vnd.ms-powerpoint",
        b"ppsm" => "application/vnd.ms-powerpoint.slideshow.macroEnabled.12",
        b"ppsx" => "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
        b"ppt" => "application/vnd.ms-powerpoint",
        b"pptm" => "application/vnd.ms-powerpoint.presentation.macroEnabled.12",
        b"pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        b"prf" => "application/pics-rules",
        b"prm" => "application/octet-stream",
        b"prx" => "application/octet-stream",
        b"ps" => "application/postscript",
        b"psd" => "application/octet-stream",
        b"psm" => "application/octet-stream",
        b"psp" => "application/octet-stream",
        b"pub" => "application/x-mspublisher",
        b"qt" => "video/quicktime",
        b"qtl" => "application/x-quicktimeplayer",
        b"qxd" => "application/octet-stream",
        b"ra" => "audio/x-pn-realaudio",
        b"ram" => "audio/x-pn-realaudio",
        b"rar" => "application/octet-stream",
        b"ras" => "image/x-cmu-raster",
        b"rf" => "image/vnd.rn-realflash",
        b"rgb" => "image/x-rgb",
        b"rm" => "application/vnd.rn-realmedia",
        b"rmi" => "audio/mid",
        b"roff" => "application/x-troff",
        b"rpm" => "audio/x-pn-realaudio-plugin",
        b"rtf" => "application/rtf",
        b"rtx" => "text/richtext",
        b"scd" => "application/x-msschedule",
        b"sct" => "text/scriptlet",
        b"sea" => "application/octet-stream",
        b"setpay" => "application/set-payment-initiation",
        b"setreg" => "application/set-registration-initiation",
        b"sgml" => "text/sgml",
        b"sh" => "application/x-sh",
        b"shar" => "application/x-shar",
        b"sit" => "application/x-stuffit",
        b"sldm" => "application/vnd.ms-powerpoint.slide.macroEnabled.12",
        b"sldx" => "application/vnd.openxmlformats-officedocument.presentationml.slide",
        b"smd" => "audio/x-smd",
        b"smi" => "application/octet-stream",
        b"smx" => "audio/x-smd",
        b"smz" => "audio/x-smd",
        b"snd" => "audio/basic",
        b"snp" => "application/octet-stream",
        b"spc" => "application/x-pkcs7-certificates",
        b"spl" => "application/futuresplash",
        b"src" => "application/x-wais-source",
        b"ssm" => "application/streamingmedia",
        b"sst" => "application/vnd.ms-pki.certstore",
        b"stl" => "application/vnd.ms-pki.stl",
        b"sv4cpio" => "application/x-sv4cpio",
        b"sv4crc" => "application/x-sv4crc",
        b"svg" => "image/svg+xml",
        b"swf" => "application/x-shockwave-flash",
        b"t" => "application/x-troff",
        b"tar" => "application/x-tar",
        b"tcl" => "application/x-tcl",
        b"tex" => "application/x-tex",
        b"texi" => "application/x-texinfo",
        b"texinfo" => "application/x-texinfo",
        b"tgz" => "application/x-compressed",
        b"thmx" => "application/vnd.ms-officetheme",
        b"thn" => "application/octet-stream",
        b"tif" => "image/tiff",
        b"tiff" => "image/tiff",
        b"toc" => "application/octet-stream",
        b"tr" => "application/x-troff",
        b"trm" => "application/x-msterminal",
        b"tsv" => "text/tab-separated-values",
        b"ttf" => "application/octet-stream",
        b"txt" => "text/plain",
        b"u32" => "application/octet-stream",
        b"uls" => "text/iuls",
        b"ustar" => "application/x-ustar",
        b"vbs" => "text/vbscript",
        b"vcf" => "text/x-vcard",
        b"vcs" => "text/plain",
        b"vdx" => "application/vnd.ms-visio.viewer",
        b"vml" => "text/xml",
        b"vsd" => "application/vnd.visio",
        b"vss" => "application/vnd.visio",
        b"vst" => "application/vnd.visio",
        b"vsto" => "application/x-ms-vsto",
        b"vsw" => "application/vnd.visio",
        b"vsx" => "application/vnd.visio",
        b"vtx" => "application/vnd.visio",
        b"wav" => "audio/wav",
        b"wax" => "audio/x-ms-wax",
        b"wbmp" => "image/vnd.wap.wbmp",
        b"wcm" => "application/vnd.ms-works",
        b"wdb" => "application/vnd.ms-works",
        b"wks" => "application/vnd.ms-works",
        b"wm" => "video/x-ms-wm",
        b"wma" => "audio/x-ms-wma",
        b"wmd" => "application/x-ms-wmd",
        b"wmf" => "application/x-msmetafile",
        b"wml" => "text/vnd.wap.wml",
        b"wmlc" => "application/vnd.wap.wmlc",
        b"wmls" => "text/vnd.wap.wmlscript",
        b"wmlsc" => "application/vnd.wap.wmlscriptc",
        b"wmp" => "video/x-ms-wmp",
        b"wmv" => "video/x-ms-wmv",
        b"wmx" => "video/x-ms-wmx",
        b"wmz" => "application/x-ms-wmz",
        b"wps" => "application/vnd.ms-works",
        b"wri" => "application/x-mswrite",
        b"wrl" => "x-world/x-vrml",
        b"wrz" => "x-world/x-vrml",
        b"wsdl" => "text/xml",
        b"wvx" => "video/x-ms-wvx",
        b"x" => "application/directx",
        b"xaf" => "x-world/x-vrml",
        b"xaml" => "application/xaml+xml",
        b"xap" => "application/x-silverlight-app",
        b"xbap" => "application/x-ms-xbap",
        b"xbm" => "image/x-xbitmap",
        b"xdr" => "text/plain",
        b"xht" => "application/xhtml+xml",
        b"xhtml" => "application/xhtml+xml",
        b"xla" => "application/vnd.ms-excel",
        b"xlam" => "application/vnd.ms-excel.addin.macroEnabled.12",
        b"xlc" => "application/vnd.ms-excel",
        b"xlm" => "application/vnd.ms-excel",
        b"xls" => "application/vnd.ms-excel",
        b"xlsb" => "application/vnd.ms-excel.sheet.binary.macroEnabled.12",
        b"xlsm" => "application/vnd.ms-excel.sheet.macroEnabled.12",
        b"xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        b"xlt" => "application/vnd.ms-excel",
        b"xltm" => "application/vnd.ms-excel.template.macroEnabled.12",
        b"xltx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
        b"xlw" => "application/vnd.ms-excel",
        b"xml" => "text/xml",
        b"xof" => "x-world/x-vrml",
        b"xpm" => "image/x-xpixmap",
        b"xps" => "application/vnd.ms-xpsdocument",
        b"xsd" => "text/xml",
        b"xsf" => "text/xml",
        b"xsl" => "text/xml",
        b"xslt" => "text/xml",
        b"xsn" => "application/octet-stream",
        b"xtp" => "application/octet-stream",
        b"xwd" => "image/x-xwindowdump",
        b"z" => "application/x-compress",
        b"zip" => "application/x-zip-compressed",
        _ => "text/plain",
    }
}
