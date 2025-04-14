fn main() {
   let x = 5;

   if x < 5 {
       println!("x is less than 5");
   } else if x == 5 {
       println!("x is equal to 5");
   } else {
       println!("x is greater than 5");
   }

   let condition = true;
   let y = if condition { 5 } else { 6 };

   println!("The value of y is: {}", y);

   loop {
       println!("again!");
   }
}
