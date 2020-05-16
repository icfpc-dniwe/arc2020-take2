#[cfg(test)]
mod test_operations {
    use crate::arc2020::types::*;
    use crate::arc2020::operations::*;
    use anyhow::Result;

    fn gen_test_image() -> Image
    {
        // returns:
        // (Y)
        //    +---+---+---+---+
        //  3 | 0 | 0 | 0 | 3 |
        //    +---+---+---+---+
        //  2 | 0 | 8 | 9 | 0 |
        //    +---+---+---+---+
        //  1 | 0 | 7 | 0 | 0 |
        //    +---+---+---+---+
        //  0 | 0 | 0 | 0 | 0 |
        //    +---+---+---+---+
        //      0   1   2   3   (X)

        let mut img = Image::default((4, 4));
        img.fill(0);
        img[(1, 1)] = 7;
        img[(1, 2)] = 8;
        img[(2, 2)] = 9;
        img[(3, 3)] = 3;
        return img;
    }

    fn option_as_result<T>(val: Option<T>) -> Result<T>
    {
        match val {
            Some(t) => Ok(t),
            None => Err(anyhow!("option is none"))
        }
    }

    #[test]
    fn test_image_block() -> Result<()>
    {
        let img = gen_test_image();
        let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
        assert_eq!(blc.pivot, (1, 1));

        let blc_data = blc.block;
        //for (p, c) in &blc_data
        //{
        //    println!("({}, {}): {}", p.0, p.1, c);
        //}

        assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(7 as u8));
        assert_eq!(option_as_result(blc_data.get(&(0, 1)))?, &(8 as u8));
        assert_eq!(option_as_result(blc_data.get(&(1, 1)))?, &(9 as u8));
        assert_eq!(option_as_result(blc_data.get(&(1, 0)))?, &(0 as u8));
        assert_eq!(blc_data.get(&(2, 2)), None);
        Ok(())
    }

    #[test]
    fn test_block_block() -> Result<()>
    {
        let img = gen_test_image();

        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            let sub = apply_block_operation(blc, &Operation::Block((1, 0), (1, 2)))?;
            let blc_data = sub.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(0 as u8));
            assert_eq!(option_as_result(blc_data.get(&(0, 1)))?, &(9 as u8));
            assert_eq!(blc_data.get(&(1, 1)), None);
            assert_eq!(sub.pivot.0, blc_pvt.0 + 1);
            assert_eq!(sub.pivot.1, blc_pvt.1 + 0);
        }

        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            let sub = apply_block_operation(blc, &Operation::Block((0, 1), (2, 1)))?;
            let blc_data = sub.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(8 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 0)))?, &(9 as u8));
            assert_eq!(blc_data.get(&(1, 1)), None);
            assert_eq!(sub.pivot.0, blc_pvt.0 + 0);
            assert_eq!(sub.pivot.1, blc_pvt.1 + 1);
        }

        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            let sub = apply_block_operation(blc, &Operation::Block((0, 1), (4, 4)))?;
            let blc_data = sub.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(8 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 0)))?, &(9 as u8));
            assert_eq!(blc_data.get(&(1, 1)), None);
            assert_eq!(sub.pivot.0, blc_pvt.0 + 0);
            assert_eq!(sub.pivot.1, blc_pvt.1 + 1);
        }

        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            let sub = apply_block_operation(blc, &Operation::Block((0, 0), (1, 1)))?;
            let blc_data = sub.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(7 as u8));
            assert_eq!(blc_data.get(&(1, 1)), None);
            assert_eq!(sub.pivot.0, blc_pvt.0);
            assert_eq!(sub.pivot.1, blc_pvt.1);
        }

        Ok(())
    }

    #[test]
    fn test_block_pivot() -> Result<()>
    {
        let img = gen_test_image();
        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            let new = apply_block_operation(blc, &Operation::Pivot((1, 1)))?;
            assert_eq!(new.pivot.0, blc_pvt.0 + 1);
            assert_eq!(new.pivot.1, blc_pvt.1 + 1);
            let blc_data = new.block;
            assert_eq!(option_as_result(blc_data.get(&(-1, -1)))?, &(7 as u8));
            assert_eq!(option_as_result(blc_data.get(&(-1, 0)))?, &(8 as u8));
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(9 as u8));
            assert_eq!(option_as_result(blc_data.get(&(0, -1)))?, &(0 as u8));
            assert_eq!(blc_data.get(&(2, 2)), None);
        }
        Ok(())
    }

    #[test]
    fn test_block_shift() -> Result<()>
    {
        let img = gen_test_image();
        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            
            let new = apply_block_operation(blc, &Operation::Shift((1, 1)))?;
            assert_eq!(new.pivot.0, blc_pvt.0 + 1);
            assert_eq!(new.pivot.1, blc_pvt.1 + 1);

            let blc_data = new.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(7 as u8));
            assert_eq!(option_as_result(blc_data.get(&(0, 1)))?, &(8 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 1)))?, &(9 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 0)))?, &(0 as u8));
            assert_eq!(blc_data.get(&(2, 2)), None);
        }
        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            
            let new = apply_block_operation(blc, &Operation::Shift((-1, -1)))?;
            assert_eq!(new.pivot.0, blc_pvt.0 - 1);
            assert_eq!(new.pivot.1, blc_pvt.1 - 1);

            let blc_data = new.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(7 as u8));
            assert_eq!(option_as_result(blc_data.get(&(0, 1)))?, &(8 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 1)))?, &(9 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 0)))?, &(0 as u8));
            assert_eq!(blc_data.get(&(2, 2)), None);
        }
        Ok(())
    }

    #[test]
    fn test_block_rotate() -> Result<()>
    {
        let img = gen_test_image();
        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            
            let new = apply_block_operation(blc, &Operation::Rotate(90.0_f64.to_radians()))?;
            assert_eq!(new.pivot.0, blc_pvt.0);
            assert_eq!(new.pivot.1, blc_pvt.1);

            let blc_data = new.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(7 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 0)))?, &(8 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, -1)))?, &(9 as u8));
            assert_eq!(option_as_result(blc_data.get(&(0, -1)))?, &(0 as u8));
            assert_eq!(blc_data.get(&(2, 2)), None);
        }
        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            let blc_pvt = blc.pivot;
            
            let new = apply_block_operation(blc, &Operation::Rotate(-90.0_f64.to_radians()))?;
            assert_eq!(new.pivot.0, blc_pvt.0);
            assert_eq!(new.pivot.1, blc_pvt.1);

            let blc_data = new.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(7 as u8));
            assert_eq!(option_as_result(blc_data.get(&(0, 1)))?, &(0 as u8));
            assert_eq!(option_as_result(blc_data.get(&(-1, 0)))?, &(8 as u8));
            assert_eq!(option_as_result(blc_data.get(&(-1, 1)))?, &(9 as u8));
            assert_eq!(blc_data.get(&(2, 2)), None);
        }
        {
            let blc = extract_block_from_image(&img, &(1, 1), 2, 2)?;
            println!("blc:");
            for (p, c) in &blc.block
            {
                println!("({}, {}): {}", p.0, p.1, c);
            }
            let blc_pvt = blc.pivot;
            
            let new = apply_block_operation(blc, &Operation::Rotate(45.0_f64.to_radians()))?;
            println!("new:");
            for (p, c) in &new.block
            {
                println!("({}, {}): {}", p.0, p.1, c);
            }
            assert_eq!(new.pivot.0, blc_pvt.0);
            assert_eq!(new.pivot.1, blc_pvt.1);

            let blc_data = new.block;
            assert_eq!(option_as_result(blc_data.get(&(0, 0)))?, &(7 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 1)))?, &(8 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, 0)))?, &(9 as u8));
            assert_eq!(option_as_result(blc_data.get(&(1, -1)))?, &(0 as u8));
            assert_eq!(blc_data.get(&(2, 2)), None);
        }
        Ok(())
    }
}