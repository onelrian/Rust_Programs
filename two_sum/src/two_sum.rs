pub fn two_sum(array:Vec<i32>,target:i32) -> Vec<Vec<i32>> {
    let mut result:Vec<Vec<i32>> = Vec::new();
    for i in 0..array.len(){
        for j in i+1..array.len(){
            if array[i]+array[j]==target{
                result.push(vec![i as i32 ,j as i32]);
            }
        }
    }
    result
}