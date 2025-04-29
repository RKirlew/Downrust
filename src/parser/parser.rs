
pub enum MarkdownType{
    H1TYPE,
    H2TYPE,
    UNTYPE,
    PARAGRAPH
}
pub struct Wrapper{
       pub md_type:MarkdownType,
       pub input:String,
      

}

impl Wrapper{
      
        pub fn value(&self)->String{ 
            self.input.to_string()
        }
        pub fn wrap(&self)->String{
            let mut result=String::new();
            if matches!(&self.md_type,MarkdownType::H1TYPE){
                result+="<h1>";
                result+=&self.input;
                result+="</h1>";
                result.to_string()
            }else if matches!(&self.md_type,MarkdownType::H2TYPE){
                result+="<h2>";
                result+=&self.input;
                result+="</h2>";
                result.to_string()
            }else if matches!(&self.md_type,MarkdownType::PARAGRAPH){
                result+="<p>";
                result+=&self.input;
                result+="</p>";
                result.to_string()
            }else{
               "Not working".to_string()
            }
        }
    }
