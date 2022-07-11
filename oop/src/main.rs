use oop::{Post, Post2};

// Implementing an Object-Oriented Design Pattern
// The state pattern is an object-oriented design pattern. The crux of the
// pattern is that a value has some internal state, which is represented by a
// set of state objects, and the value’s behavior changes based on the internal
// state. The state objects share functionality: in Rust, of course, we use
// structs and traits rather than objects and inheritance. Each state object is
// responsible for its own behavior and for governing when it should change into
// another state. The value that holds a state object knows nothing about the
// different behavior of the states or when to transition between states.

// Using the state pattern means when the business requirements of the program
// change, we won’t need to change the code of the value holding the state or
// the code that uses the value. We’ll only need to update the code inside one
// of the state objects to change its rules or perhaps add more state objects.
// Let’s look at an example of the state design pattern and how to use it in
// Rust.

// We’ll implement a blog post workflow in an incremental way. The blog’s final
// functionality will look like this:
//  A blog post starts as an empty draft.
//  When the draft is done, a review of the post is requested.
//  When the post is approved, it gets published.
//  Only published blog posts return content to print, so unapproved posts can’t
//      accidentally be published.

// Any other changes attempted on a post should have no effect. For example, if
// we try to approve a draft blog post before we’ve requested a review, the post
// should remain an unpublished draft.

fn main() {
    // We want to allow the user to create a new draft blog post with Post::new.
    // We want to allow text to be added to the blog post.
    let mut post = Post::new();

    // If we try to get the post’s content immediately, before approval, we
    // shouldn't get any text because the post is still a draft. We’ve added
    // assert_eq! in the code for demonstration purposes. An excellent unit test
    // for this would be to assert that a draft blog post returns an empty
    // string from the content method, but we’re not going to write tests for
    // this example.
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // Next, we want to enable a request for a review of the post, and we want
    // content to return an empty string while waiting for the review.
    post.request_review();
    assert_eq!("", post.content());

    // When the post receives approval, it should get published, meaning the
    // text of the post will be returned when content is called.
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // Notice that the only type we’re interacting with from the crate is the
    // Post type. This type will use the state pattern and will hold a value
    // that will be one of three state objects representing the various states a
    // post can be in—draft, waiting for review, or published. Changing from one
    // state to another will be managed internally within the Post type. The
    // states change in response to the methods called by our library’s users on
    // the Post instance, but they don’t have to manage the state changes
    // directly. Also, users can’t make a mistake with the states, like
    // publishing a post before it’s reviewed.

    // Trade-offs of the State Pattern
    // We’ve shown that Rust is capable of implementing the object-oriented
    // state pattern to encapsulate the different kinds of behavior a post
    // should have in each state. The methods on Post know nothing about the
    // various behaviors. The way we organized the code, we have to look in only
    // one place to know the different ways a published post can behave: the
    // implementation of the State trait on the Published struct.

    // If we were to create an alternative implementation that didn’t use the
    // state pattern, we might instead use match expressions in the methods on
    // Post or even in the main code that checks the state of the post and
    // changes behavior in those places. That would mean we would have to look
    // in several places to understand all the implications of a post being in
    // the published state! This would only increase the more states we added:
    // each of those match expressions would need another arm.

    // With the state pattern, the Post methods and the places we use Post don’t
    // need match expressions, and to add a new state, we would only need to add
    // a new struct and implement the trait methods on that one struct.

    // The implementation using the state pattern is easy to extend to add more
    // functionality. To see the simplicity of maintaining code that uses the
    // state pattern, try a few of these suggestions:
    //  Add a reject method that changes the post’s state from PendingReview
    //      back to Draft.
    //  Require two calls to approve before the state can be changed to
    //      Published.
    //  Allow users to add text content only when a post is in the Draft state.
    //      Hint: have the state object responsible for what might change about
    //      the content but not responsible for modifying the Post.

    // One downside of the state pattern is that, because the states implement
    // the transitions between states, some of the states are coupled to each
    // other. If we add another state between PendingReview and Published, such
    // as Scheduled, we would have to change the code in PendingReview to
    // transition to Scheduled instead. It would be less work if PendingReview
    // didn’t need to change with the addition of a new state, but that would
    // mean switching to another design pattern.

    // Another downside is that we’ve duplicated some logic. To eliminate some
    // of the duplication, we might try to make default implementations for the
    // request_review and approve methods on the State trait that return self;
    // however, this would violate object safety, because the trait doesn’t know
    // what the concrete self will be exactly. We want to be able to use State
    // as a trait object, so we need its methods to be object safe.

    // Other duplication includes the similar implementations of the
    // request_review and approve methods on Post. Both methods delegate to the
    // implementation of the same method on the value in the state field of
    // Option and set the new value of the state field to the result. If we had
    // a lot of methods on Post that followed this pattern, we might consider
    // defining a macro to eliminate the repetition.

    // By implementing the state pattern exactly as it’s defined for
    // object-oriented languages, we’re not taking as full advantage of Rust’s
    // strengths as we could. Let’s look at some changes we can make to the blog
    // crate that can make invalid states and transitions into compile time
    // errors.

    // Encoding States and Behavior as Types
    // We’ll show you how to rethink the state pattern to get a different set of
    // trade-offs. Rather than encapsulating the states and transitions
    // completely so outside code has no knowledge of them, we’ll encode the
    // states into different types. Consequently, Rust’s type checking system
    // will prevent attempts to use draft posts where only published posts are
    // allowed by issuing a compiler error.
    let mut post = Post2::new();

    // The changes we needed to make to main to reassign post mean that this
    // implementation doesn’t quite follow the object-oriented state pattern
    // anymore: the transformations between the states are no longer
    // encapsulated entirely within the Post implementation. However, our gain
    // is that invalid states are now impossible because of the type system and
    // the type checking that happens at compile time! This ensures that certain
    // bugs, such as display of the content of an unpublished post, will be
    // discovered before they make it to production.
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

    // We’ve seen that even though Rust is capable of implementing
    // object-oriented design patterns, other patterns, such as encoding state
    // into the type system, are also available in Rust. These patterns have
    // different trade-offs. Although you might be very familiar with
    // object-oriented patterns, rethinking the problem to take advantage of
    // Rust’s features can provide benefits, such as preventing some bugs at
    // compile time. Object-oriented patterns won’t always be the best solution
    // in Rust due to certain features, like ownership, that object-oriented
    // languages don’t have.
}
