fn main() {
    
    let string_list = vec!["burak".to_string(),"zeynep".to_string(),"sezgin".to_string()];
    let str_filt = FilterCondition {
        value: "burak".to_string()
    };

    let result_str_list = custom_filter(string_list, &str_filt);

    println!("{:?}",result_str_list);

    let number_list = vec![1,1,1,2,3,4,4];

    let num_filt = FilterCondition {value: 4};

    let result_num_list = custom_filter(number_list, &num_filt);

    println!("{:?}",result_num_list);





}

fn custom_filter<T>(list: Vec<T>, filt: &FilterCondition<T>) -> Vec<T> where T: PartialEq{

    let mut new_list: Vec<T> = Vec::new();
    for item in list {
        if filt.is_match(&item) {
            new_list.push(item);
        };
    };
    new_list

}

struct FilterCondition<T> {
    value: T
}


impl<T: PartialEq> FilterCondition<T> {
    
    fn is_match(&self, inp: &T) -> bool {
        inp == &self.value
    }
}