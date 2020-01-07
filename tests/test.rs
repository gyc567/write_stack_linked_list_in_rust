extern crate lists;
extern crate speculate;

use lists::my_stack::*;
use speculate::speculate;

speculate! {

    describe "testing  my stack " {

        before {
        }

        it "should  be empty"{
        let mut stack = MyStack::<u32>::with_capacity(10);
        assert_eq!(10, stack.capacity());
        assert_eq!(true, stack.is_empty());
        }
        it "test peek fn should work fine "{
        let mut stack = MyStack::<u32>::with_capacity(10);
        stack.push(1u32);
        assert_eq!(Some(&1u32), stack.peek());
        stack.push(2u32);
        stack.push(3u32);
        assert_eq!(Some(&3u32), stack.peek());
        }
        it "test pop and push fn should work fine "{
        let mut stack = MyStack::<u32>::with_capacity(10);
        stack.push(1u32);
        assert_eq!(false, stack.is_empty());
        assert_eq!(Some(1u32), stack.pop());
        assert_eq!(None, stack.pop());
        assert_eq!(true, stack.is_empty());
        }
        it "test pop peek with maxSize fn should work fine "{
        let mut stack = MyStack::<u32>::with_capacity(1);
        assert_eq!(true, stack.push(1u32));
        assert_eq!(false, stack.is_empty());
         assert_eq!(Some(&1u32), stack.peek());
        assert_eq!(false, stack.push(2u32));
        assert_eq!(Some(1u32), stack.pop());
        assert_eq!(None, stack.pop());
        assert_eq!(true, stack.is_empty());
        }
         it "test size fn should work fine "{
        let mut stack = MyStack::<u32>::with_capacity(1);
        assert_eq!(0, stack.size());
        stack.push(1u32);
        assert_eq!(1, stack.size());
        }
    }
}
