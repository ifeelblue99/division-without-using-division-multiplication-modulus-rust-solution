struct Division {
  value: usize,
  remainder: isize,
}

fn main() {
  if let Some(res) = divide(10, 3){
    println!("result. {} - remainder. {}", res.value, res.remainder);
  } else {
    println!("Invalid parameters!");
  }
}

fn divide(mut number: isize,divisor: isize) -> Option<Division> {
  let mut counter:usize = 0; 
  
  if divisor == 0 || divisor > number{
    return None
  }
  loop {
    if number - divisor == 0 {
      counter = counter + 1;
      return Some(Division{value: counter, remainder: 0}) 
    } 
    else if number - divisor < 0 {
      return Some(Division{value: counter, remainder: number})       
    }
    number = number - divisor;
    counter = counter + 1;
  }
}
