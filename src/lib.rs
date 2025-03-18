pub mod heap {
    pub struct Heap<'a>{
        pub arr: &'a mut[i32],
        pub size: usize,
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn left(i: usize) -> usize{
        2 * i + 1
    }

    fn right(i: usize) -> usize{
        2 * i + 2
    }

    pub fn swap_node(heap: &mut Heap, i : usize, j : usize){
        let tmp: i32 = heap.arr[i];
        heap.arr[i] = heap.arr[j];
        heap.arr[j] = tmp;
    }

    pub fn min_heapify(heap: &mut Heap, i: usize) {
        let l: usize = left(i);
        let r: usize = right(i);
        let mut smallest: usize = i;

        if l < heap.size && heap.arr[l] < heap.arr[smallest] {
            smallest = l;
        }

        if r < heap.size && heap.arr[r] < heap.arr[smallest] {
            smallest = r;
        }

        if smallest != i {
            //swap nodes
            swap_node(heap, i, smallest);
            //call recursively min heapify
            min_heapify(heap, smallest);
        }
    }

    pub fn build_min_heap(heap: &mut Heap) {
        heap.size = heap.arr.len();
        for i in (0..heap.arr.len() / 2).rev() {
            min_heapify(heap, i);
        }
    }

    pub fn heap_sort(heap: &mut Heap) {
        build_min_heap(heap);
        for i in (1..heap.arr.len()).rev() {
            swap_node(heap, 0, i);
            heap.size -= 1;
            min_heapify(heap, 0);
        }
    }

    pub fn display_heap(heap: &Heap) {
        println!("{:?}", heap.arr);
    }

}
