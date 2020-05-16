use super::types::*;
use anyhow::Result;
use math;
use math::round;
use std;
use std::cmp;
use std::collections::HashMap;
use std::collections::LinkedList;

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
    ExtractObjects,
}

pub fn apply_block(mut blc: ImageBlock, pivot: &Point, w: u64, h: u64) -> Result<ImageBlock>
{
    assert!(pivot.0 >= 0);
    assert!(pivot.1 >= 0);
    let mut new_block : HashMap<Point, u8> = HashMap::new();
    for ((x, y), color) in &blc.block
    {
        //println!("pivot : {}, {}, w: {} h: {}", pivot.0, pivot.1, w, h);
        //println!("x: {}, y: {}", *x, *y);
        if *x >= pivot.0 && *x < (pivot.0 + (w as i64)) && *y >= pivot.1 && *y < (pivot.1 + (h as i64))
        {
            //println!("inserting: ({}, {}) = {} at original ({}, {})", *x - pivot.0, *y - pivot.1, *color, *x, *y);
            new_block.insert((*x - pivot.0, *y - pivot.1), *color);
        }
    }
    blc.block = new_block;
    blc.pivot = (blc.pivot.0 + pivot.0, blc.pivot.1 + pivot.1);
    Ok(blc)
}

pub fn apply_pivot(mut blc: ImageBlock, pivot: &Point) -> Result<ImageBlock>
{
    let mut new_block : HashMap<Point, u8> = HashMap::new();
    for ((x, y), color) in &blc.block
    {
        new_block.insert((*x - pivot.0, *y - pivot.1), *color);
    }
    blc.block = new_block;
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

    let mut new_block : HashMap<Point, u8> = HashMap::new();
    for ((x, y), color) in &blc.block
    {
        let x_new = c * (*x as f64) + s * (*y as f64);
        let y_new = -s * (*x as f64) + c * (*y as f64);
        println!("x_new: {}, y_new: {}, color: {}", x_new, y_new, *color);
        let new_x = (round::half_up(x_new, 0)) as i64;
        let new_y = (round::half_up(y_new, 0)) as i64;
        new_block.insert((new_x, new_y), *color);
    }
    blc.block = new_block;
    Ok(blc)
}

pub fn apply_change_color(mut blc: ImageBlock, func: fn (u8) -> u8) -> Result<ImageBlock>
{
    let mut fuck_rust : HashMap<Point, u8> = HashMap::new();
    for ((x, y), color) in &blc.block
    {
        fuck_rust.insert((*x, *y), func(*color));
    }

    for (p, c) in fuck_rust
    {
        blc.block.insert(p, c);
    }

    Ok(blc)
}

pub fn apply_change_color_unconditionally(mut blc: ImageBlock, color: u8) -> Result<ImageBlock>
{
    let mut fuck_rust : HashMap<Point, u8> = HashMap::new();
    for ((x, y), _) in &blc.block
    {
        fuck_rust.insert((*x, *y), color);
    }

    for (p, c) in fuck_rust
    {
        blc.block.insert(p, c);
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

    let mut fuck_rust : HashMap<Point, u8> = HashMap::new();
    for ((x, y), color) in &blc.block
    {
        fuck_rust.insert((x_max - *x, *y), *color);
        fuck_rust.insert((*x, *y), blc.block[&(x_max - *x, *y)]);
    }

    for (p, c) in fuck_rust
    {
        blc.block.insert(p, c);
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

    let mut fuck_rust : HashMap<Point, u8> = HashMap::new();
    for ((x, y), color) in &blc.block
    {
        fuck_rust.insert((*x, y_max - *y), *color);
        fuck_rust.insert((*x, *y), blc.block[&(*x, y_max - *y)]);
    }

    for (p, c) in fuck_rust
    {
        blc.block.insert(p, c);
    }

    Ok(blc)
}

pub fn apply_filter_by(mut blc: ImageBlock, pred: fn (Point, u8) -> bool) -> Result<ImageBlock>
{
    let mut new_block : HashMap<Point, u8> = HashMap::new();
    for ((x, y), color) in &blc.block
    {
        if pred((*x, *y), *color)
        {
            new_block.insert((*x, *y), *color);
        }
    }
    blc.block = new_block;
    Ok(blc)
}

pub fn apply_extract_objects(_blc: ImageBlock) -> Result<ImageBlock>
{
    Err(anyhow!("not implemented"))
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
        Operation::ExtractObjects => apply_extract_objects(blc),
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
        img[(*y as usize, *x as usize)] = *color;
    }
    Ok(img)
}

pub fn apply_image_operations(img: Image, ops: &LinkedList<Operation>) -> Result<Image>
{
    let mut cur_block = extract_block_from_image(&img, &(0, 0), img.shape()[0] as u64, img.shape()[1] as u64);
    for op in ops
    {
        match cur_block {
            Ok(blc) => cur_block = apply_block_operation(blc, &op),
            Err(msg) => return Err(msg),
        }
    }

    match cur_block {
        Ok(blc) => return commit_block_to_image(img, &blc),
        Err(msg) => return Err(msg),
    }
}