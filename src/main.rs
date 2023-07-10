fn main() {
    
    let input_list = vec!["burak".to_string(),"zeynep".to_string(),"sezgin".to_string()];
    let filter = FilterCondition {
        text: "burak".to_string()
    };

    let result_list = custom_filter(input_list, &filter);

    println!("{:?}",result_list);





}

fn custom_filter(list: Vec<String>, filt: &FilterCondition) -> Vec<String> {

    let mut new_list: Vec<String> = Vec::new();
    for item in list {
        if filt.is_match(&item) {
            new_list.push(item);
        };
    };
    new_list

}

struct FilterCondition {
    text: String
}


impl FilterCondition {
    
    fn is_match(&self, inp: &str) -> bool {
        inp == self.text
    }
}