#[derive(Copy,Clone)]
struct dist_point
{
    dist : u32,
    from:usize,        
}

pub fn dijkstra_shortest_path(path :[[u32;6];6] , mut from:usize)
{
    println!("in the function");
    let start_point = from;

    if from > path.len()
    {
        return;
    }
    let mut arr_dist_length =vec![dist_point{dist:<u32>::max_value(),from:<usize>::max_value()};path.len()];
    let mut isVisited = vec![false;path.len()];
    let mut visited_num=1;
    while visited_num < isVisited.len()
    {
        let mut min_val=<u32>::max_value();
        visited_num = visited_num+1;
        for (index,ref dp) in arr_dist_length.iter().enumerate()
        {
            if !isVisited[index]
            {
                if min_val > dp.dist{
                    min_val=dp.dist;
                    from=index;
                }
            }
        }
        isVisited[from] = true;
        
        for (_node, _dis) in path[from].iter().enumerate()
        {
            if _node == from  
            {
                  continue;
            }
            if *_dis == <u32>::max_value()
            {
                continue;
            }
            else if arr_dist_length[from].dist != <u32>::max_value() && 
             arr_dist_length[_node].dist > *_dis +  arr_dist_length[from].dist
            {
                arr_dist_length[_node].dist = *_dis +  arr_dist_length[from].dist;
                arr_dist_length[_node].from = from;
            }

        }
        let index=0;
        arr_dist_length.iter_mut().for_each(|dp|
        {
            println!("from: {}, distance: {}",dp.from,dp.dist );
        });


    }
}