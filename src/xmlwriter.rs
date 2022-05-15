use std::fs::File;
use std::io::Write;

static RIGHT_BRACKET: &str = ">";
static LEFT_BRACKET: &str = "<";
static WHITESPACE: &str = " ";

pub struct XmlWriter {
    pub xmlfile: File,
}

impl XmlWriter {
    /// Opens a jack file and gets ready to tokenize it
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the jack file, excluding the fileextention
    ///
    /// # Returns
    ///
    /// * This self xmlwriter object
    pub fn new(path: &String) -> Self {
        let mut xmlwriter = XmlWriter {
            xmlfile: File::create(path.to_owned().split(".jack").collect::<Vec<_>>()[0].to_owned() + "MyT.xml").unwrap(),
        };
        xmlwriter.xmlfile.write("<tokens>\n".as_ref());
        xmlwriter
    }

    /// Writes to an xml file
    ///
    /// # Arguments
    ///
    /// * `tag` - the tag in xml
    /// * `content` - the content of the tag
    pub fn write(&mut self, tag: String, content: String) {
        let opening_tag = LEFT_BRACKET.to_string() + tag.as_str() + RIGHT_BRACKET;
        let closing_tag = LEFT_BRACKET.to_string() + "/" + tag.as_str() + RIGHT_BRACKET;
        self.xmlfile.write((opening_tag + WHITESPACE + content.as_str() + WHITESPACE + closing_tag.as_str() + "\n").as_ref());
    }

    /// Destructor for xmlwriter in order to write the closing bracket
    pub fn write_last(&mut self) {
        self.xmlfile.write((LEFT_BRACKET.to_string() + "/" + "tokens" + RIGHT_BRACKET).as_ref());
    }
}