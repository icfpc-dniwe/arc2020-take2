use super::types::*;
use super::operations::*;
use anyhow::Result;
use std;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::iter::FromIterator;

pub fn extract_objects(img: &Image, bg_color: u8, pred_neigh: fn (&(usize, usize), &(usize, usize)) -> bool) -> Result<Vec<Vec<(usize, usize)>>>
{
    let mut blocks : HashMap<u8, LinkedList<HashSet<(usize, usize)>>> = HashMap::new();
    
    // for each color extract one or more sets of neighbours
    // after this pass we can still have 2 separate sets of neighbours for object parts
    // these will be merged later on second pass
    println!("first pass:");
    for (p, c) in img.indexed_iter()
    {
        if *c == bg_color
        {
            continue;
        }

        if blocks.get(c) == None
        {
            blocks.insert(*c, LinkedList::new());
        }

        let cur_list = blocks.get_mut(c).unwrap();
        //println!("  current point: {:?}, color: {}", p, c);
        //println!("  current list: {:?}", cur_list);

        let mut found = false;
        for set in cur_list.iter_mut()
        {
            //println!("    set: {:?}", set);
            for pt in set.iter()
            {
                //println!("      pred neigh: {:?}, {:?} -> {}", &p, pt, pred_neigh(&p, pt));
                if pred_neigh(&p, pt)
                {
                    found = true;
                    break;
                }
            }

            if found
            {
                set.insert(p);
                break;
            }
        }

        if !found
        {
            let mut new_set = HashSet::new();
            new_set.insert(p);
            cur_list.push_back(new_set);
        }
    }
    //println!("{:?}", blocks);

    // merge sets, that relate to the same object and obtain only sets corresponding to separate objects of different color
    //println!("second pass:");
    let mut objs = LinkedList::new();
    for (_, obj_set_list) in blocks.iter_mut()
    {
        //println!("  current color: {}", c);
        //println!("  current list:");
        //println!("  {:?}", obj_set_list);
        let mut prev_object_len = 0;
        let mut cur_obj = HashSet::new();
        loop
        {
            //println!("    current object:");
            //println!("    {:?}", &cur_obj);
            for set in obj_set_list.iter_mut()
            {
                //println!("      current set:");
                //println!("      {:?}", *set);
                let mut merge = false;
                if cur_obj.len() > 0
                {
                    for pt0 in set.iter()
                    {
                        for pt1 in cur_obj.iter()
                        {
                            //println!("      pred neigh: {:?}, {:?} -> {}", pt0, pt1, pred_neigh(pt0, pt1));
                            if pred_neigh(pt0, pt1)
                            {
                                merge = true;
                                break;
                            }
                        } 
                    }
                }
                else
                {
                    merge = true;    
                }

                //println!("      merge: {}", merge);
                if merge
                {
                    for elem in set.drain()
                    {
                        cur_obj.insert(elem);
                    }
                }
            }

            //println!("      prev object len {}", prev_object_len);
            //println!("      cur object len {}", cur_obj.len());
            if cur_obj.len() != 0
            {
                if cur_obj.len() == prev_object_len
                {
                    //println!("      add object to output: {:?}", cur_obj);
                    objs.push_back(Vec::from_iter(cur_obj.into_iter()));
                    cur_obj = HashSet::new()
                }
                prev_object_len = cur_obj.len();
            }
            else
            {
                break;
            }
        }
    }

    Ok(Vec::from_iter(objs.into_iter()))
}

pub fn get_bounding_box(obj: &Vec<(usize, usize)>) -> Result<BoundingBox>
{
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    for pt in obj.iter()
    {
        x_min = cmp::min(pt.0, x_min);
        x_max = cmp::max(pt.0, x_max);
        y_min = cmp::min(pt.1, y_min);
        y_max = cmp::max(pt.1, y_max);
    }
    Ok(((x_min, y_min), (x_max, y_max)))
}

pub fn get_image_block_by_bounding_box(img: &Image, bb: &BoundingBox) -> Result<ImageBlock>
{
    extract_block_from_image(img, &((bb.0).0 as i64, (bb.0).1 as i64), ((bb.1).0 - (bb.0).0) as u64, ((bb.1).1 - (bb.0).1) as u64)
}