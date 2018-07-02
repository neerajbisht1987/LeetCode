
pub fn two_sum(vec :&Vec<i32>, sum:i32)-> Option<(usize,usize)>
{
    
    let mut m_vec = vec.clone();
    m_vec.sort();
    let mut begin =0;
    let mut end= m_vec.len()-1;
    while begin < end 
    {
        if m_vec[begin]+m_vec[end] < sum
        {
            begin= begin+1;
        }
        else if m_vec[begin]+m_vec[end] > sum
        {
            end=end-1;
        }
        else
        {
            
            let x=(begin,end);
            return Some(x);
        }
    }
    None

}