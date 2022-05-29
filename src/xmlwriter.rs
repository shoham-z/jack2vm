use std::fs::File;
use std::io::Write;

static RIGHT_BRACKET: &str = ">";
static LEFT_BRACKET: &str = "<";

pub struct XmlWriter {
    pub xmlfile: File,
}

impl XmlWriter {
    /// Creates a new xml file and gets ready to write to it
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the jack file, including the file extension
    ///
    /// # Returns
    ///
    /// * This self xmlwriter object
    pub fn new(path: &String) -> Self {
        XmlWriter {
            xmlfile: File::create(path.to_owned().split(".jack").collect::<Vec<_>>()[0].to_owned() + ".xml").unwrap(),
        }
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
        self.xmlfile.write((opening_tag + content.as_str() + closing_tag.as_str() + "\n").as_ref()).expect("ERROR WRITING TOKENS");
    }

    /// Writes an opening tag
    pub fn open_tag(&mut self, tag:String){
        self.xmlfile.write(("<".to_owned() + &tag + ">\n".as_ref()).as_ref()).expect("ERROR WRITING TOKENS");
    }

    /// Writes a closing tag
    pub fn close_tag(&mut self, tag:String) {
        self.xmlfile.write(("</".to_owned() + &tag + ">\n".as_ref()).as_ref()).expect("ERROR WRITING TOKENS");
    }
}