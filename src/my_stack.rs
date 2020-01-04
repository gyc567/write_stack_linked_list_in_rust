/*
stack in java:
public class MyStack {
   private int maxSize;
   private long[] stackArray;
   private int top;
   public MyStack(int s) {
      maxSize = s;
      stackArray = new long[maxSize];
      top = -1;
   }
   public void push(long j) {
      stackArray[++top] = j;
   }
   public long pop() {
      return stackArray[top--];
   }
   public long peek() {
      return stackArray[top];
   }
   public boolean isEmpty() {
      return (top == -1);
   }
   public boolean isFull() {
      return (top == maxSize - 1);
   }
   public static void main(String[] args) {
      MyStack theStack = new MyStack(10);
      theStack.push(10);
      theStack.push(20);
      theStack.push(30);
      theStack.push(40);
      theStack.push(50);
      while (!theStack.isEmpty()) {
         long value = theStack.pop();
         System.out.print(value);
         System.out.print(" ");
      }
      System.out.println("");
   }
}
*/
///

pub struct MyStack<T> {
   maxSize: usize,
   items: Vec<T>,
}
impl<T> MyStack<T> {
   pub fn with_capacity(maxSize: usize) -> MyStack<T> {
      MyStack {
         maxSize: maxSize,
         items: Vec::with_capacity(maxSize),
      }
   }
   pub fn capacity(&self) -> usize {
      self.items.capacity()
   }
   pub fn is_empty(&mut self) -> bool {
      self.items.is_empty()
   }
   pub fn pop(&mut self) -> Option<T> {
      self.items.pop()
   }
   pub fn peek(&self) -> Option<&T> {
      self.items.last()
   }
   pub fn size(&self) -> usize {
      self.items.len()
   }
   pub fn push(&mut self, item: T) -> bool {
      if self.items.len() == self.maxSize {
         return false;
      }
      self.items.push(item);
      return true;
   }
}
