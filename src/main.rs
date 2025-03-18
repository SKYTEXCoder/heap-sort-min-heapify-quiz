extern crate heap_sort_min_heapify_quiz;

use heap_sort_min_heapify_quiz::heap;

fn main() {

  println!("");

   let mut arr: [i32; 10] = [4,1,3,2,16,9,10,14,8,7];
   println!("Original Array: {:?}",arr);
   let mut alen = arr.len();
   let mut heap : heap::Heap = heap::Heap{arr: &mut arr, size: alen};
   heap::swap_node(&mut heap, 0,1);
   println!("After Swapping Index 0 and 1: {:?}", heap.arr);

  let mut arr = [16,4,10,14,7,9,3,2,8,1];
  alen = arr.len();
  heap = heap::Heap{arr: &mut arr, size: alen};
  heap::min_heapify(&mut heap, 0);
  println!("After Maximum Heapifying: {:?}", heap.arr);

  let mut arr = [4,1,3,2,16,9,10,14,8,7];
  alen = arr.len();
  heap = heap::Heap{arr: &mut arr, size: alen};
  heap::build_max_heap(&mut heap);
  println!("After Building The Max Heap: {:?}", heap.arr);

  let mut arr = [4,1,3,2,16,9,10,14,8,7, 10, 15, 11, 13, 29, 100, 11, 14, 5, 0, 1, -7, 4, 299, 87, 39, 41, 99, 31, 54, 31, 39, 77, 69, 68, 70];
  alen = arr.len();
  heap = heap::Heap{arr: &mut arr, size: alen};
  heap::heap_sort(&mut heap);
  println!("After Heap Sorting: {:?}", heap.arr);

  println!("");
}
