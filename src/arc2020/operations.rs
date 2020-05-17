use super::types::*;
use anyhow::Result;
use math;
use math::round;
use std;
use std::cmp;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::iter::FromIterator;

pub type Point = (i64, i64);
pub type Vector = (i64, i64);

pub struct ImageBlock {
    pub block: HashMap<Point, u8>,
    pub pivot: Point,
}

pub enum Operation {
    Block(Point, (u64, u64)),
    Pivot(Point),
    Shift(Vector),
    Rotate(f64),
    ChangeColor(fn (u8) -> u8),
    ChangeColorUnconditionally(u8),
    FilterBy(fn (Point, u8) -> bool),
    FlipX,
    FlipY,
}

pub fn apply_block(mut blc: ImageBlock, pivot: &Point, w: u64, h: u64) -> Result<ImageBlock>
{
    // rust is just impossibly stubborn
    let mut tmp_map : HashMap<Point, u8> = HashMap::from_iter(blc.block.drain());
    for ((x, y), color) in tmp_map.drain()
    {
        if x >= pivot.0 && x < (pivot.0 + (w as i64)) && y >= pivot.1 && y < (pivot.1 + (h as i64))
        {
            blc.block.insert((x - pivot.0, y - pivot.1), color);
        }
    }
    blc.pivot = (blc.pivot.0 + pivot.0, blc.pivot.1 + pivot.1);
    Ok(blc)
}

pub fn apply_pivot(mut blc: ImageBlock, pivot: &Point) -> Result<ImageBlock>
{
    // rust is just impossibly stubborn
    let mut tmp_map : HashMap<Point, u8> = HashMap::from_iter(blc.block.drain());
    for ((x, y), color) in tmp_map.drain()
    {
        blc.block.insert((x - pivot.0, y - pivot.1), color);
    }
    blc.pivot = (blc.pivot.0 + pivot.0, blc.pivot.1 + pivot.1);
    Ok(blc)
}

pub fn apply_shift(mut blc: ImageBlock, v: &Vector) -> Result<ImageBlock>
{
    blc.pivot.0 += v.0;
    blc.pivot.1 += v.1;
    Ok(blc)
}

pub fn apply_rotate(mut blc: ImageBlock, angle: f64) -> Result<ImageBlock>
{
    let c = angle.cos();
    let s = angle.sin();

    // rust is just impossibly stubborn
    let mut tmp_map : HashMap<Point, u8> = HashMap::from_iter(blc.block.drain());
    for ((x, y), color) in tmp_map.drain()
    {
        let x_new = c * (x as f64) + s * (y as f64);
        let y_new = -s * (x as f64) + c * (y as f64);
        let new_x = (round::half_up(x_new, 0)) as i64;
        let new_y = (round::half_up(y_new, 0)) as i64;
        blc.block.insert((new_x, new_y), color);
    }

    Ok(blc)
}

pub fn apply_change_color(mut blc: ImageBlock, func: fn (u8) -> u8) -> Result<ImageBlock>
{
    for (_, color) in blc.block.iter_mut()
    {
        *color = func(*color);
    }

    Ok(blc)
}

pub fn apply_change_color_unconditionally(mut blc: ImageBlock, color: u8) -> Result<ImageBlock>
{
    for (_, _color) in blc.block.iter_mut()
    {
        *_color = color;
    }

    Ok(blc)
}

pub fn apply_flip_x(mut blc: ImageBlock) -> Result<ImageBlock>
{
    let mut x_max = blc.pivot.0;
    for ((x, _), _) in &blc.block
    {
        x_max = cmp::max(x_max, *x);
    }

    // rust is just impossibly stubborn
    let mut tmp_map : HashMap<Point, u8> = HashMap::from_iter(blc.block.drain());
    for ((x, y), color) in tmp_map.drain()
    {
        blc.block.insert((x_max - x, y), color);
    }

    Ok(blc)
}

pub fn apply_flip_y(mut blc: ImageBlock) -> Result<ImageBlock>
{
    let mut y_max = blc.pivot.1;
    for ((_, y), _) in &blc.block
    {
        y_max = cmp::max(y_max, *y);
    }

    // rust is just impossibly stubborn
    let mut tmp_map : HashMap<Point, u8> = HashMap::from_iter(blc.block.drain());
    for ((x, y), color) in tmp_map.drain()
    {
        blc.block.insert((x, y_max - y), color);
    }

    Ok(blc)
}

pub fn apply_filter_by(mut blc: ImageBlock, pred: fn (Point, u8) -> bool) -> Result<ImageBlock>
{
    let mut to_remove = LinkedList::new();
    for ((x, y), color) in blc.block.iter()
    {
        if !pred((*x, *y), *color)
        {
            to_remove.push_back((*x, *y));
        }
    }
    
    for p in to_remove
    {
        blc.block.remove(&p);
    }

    Ok(blc)
}

pub fn apply_block_operation(blc: ImageBlock, op: &Operation) -> Result<ImageBlock>
{
    match op
    {
        Operation::Block(p, (w, h)) => apply_block(blc, p, *w, *h),
        Operation::Pivot(p) => apply_pivot(blc, p),
        Operation::Shift(vec) => apply_shift(blc, vec),
        Operation::Rotate(angle) => apply_rotate(blc, *angle),
        Operation::ChangeColor(func) => apply_change_color(blc, *func),
        Operation::ChangeColorUnconditionally(color) => apply_change_color_unconditionally(blc, *color),
        Operation::FilterBy(pred) => apply_filter_by(blc, *pred),
        Operation::FlipX => apply_flip_x(blc),
        Operation::FlipY => apply_flip_y(blc),
    }
}

pub fn extract_block_from_image(src: &Image, pivot: &Point, w: u64, h: u64) -> Result<ImageBlock>
{
    let mut blc = ImageBlock {
        pivot: *pivot,
        block: HashMap::new(),
    };
    for x in pivot.0..(pivot.0 + (w as i64))
    {
        for y in pivot.1..(pivot.1 + (h as i64))
        {
            blc.block.insert((x - pivot.0, y - pivot.1), src[(x as usize, y as usize)]);
        }
    }
    Ok(blc)
}

pub fn commit_block_to_image(mut img: Image, blc: &ImageBlock) -> Result<Image>
{
    for ((x, y), color) in &blc.block
    {
        let img_x = x + blc.pivot.0;
        let img_y = y + blc.pivot.1;
        assert!(img_y >= 0 && (img_y as usize) < img.nrows());
        assert!(img_x >= 0 && (img_x as usize) < img.ncols());
        img[(img_x as usize, img_y as usize)] = *color;
    }
    Ok(img)
}

pub fn apply_image_operations(img: Image, ops: &LinkedList<Operation>) -> Result<Image>
{
    let mut cur_block = extract_block_from_image(&img, &(0, 0), img.shape()[0] as u64, img.shape()[1] as u64);
    for op in ops
    {
        cur_block = apply_block_operation(cur_block?, &op);
    }

    commit_block_to_image(img, &cur_block?)
}