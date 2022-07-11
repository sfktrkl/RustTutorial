// Defining Post and Creating a New Instance in the Draft State
// Let’s get started on the implementation of the library! We know we need a
// public Post struct that holds some content, so we’ll start with the
// definition of the struct and an associated public new function to create an
// instance of Post.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // When we create a new Post, we set its state field to a Some value that
    // holds a Box. This Box points to a new instance of the Draft struct. This
    // ensures whenever we create a new instance of Post, it will start out as a
    // draft. Because the state field of Post is private, there is no way to
    // create a Post in any other state! In the Post::new function, we set the
    // content field to a new, empty String.
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // Storing the Text of the Post Content
    // We want to be able to call a method named add_text and pass it a &str
    // that is then added to the text content of the blog post. We implement
    // this as a method rather than exposing the content field as pub. This
    // means we can implement a method later that will control how the content
    // field’s data is read.

    // The add_text method takes a mutable reference to self, because we’re
    // changing the Post instance that we’re calling add_text on. We then call
    // push_str on the String in content and pass the text argument to add to
    // the saved content. This behavior doesn’t depend on the state the post is
    // in, so it’s not part of the state pattern. The add_text method doesn’t
    // interact with the state field at all, but it is part of the behavior we
    // want to support.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Ensuring the Content of a Draft Post Is Empty
    // Even after we’ve called add_text and added some content to our post, we
    // still want the content method to return an empty string slice because the
    // post is still in the draft state.
    pub fn content(&self) -> &str {
        // Because the goal is to keep all these rules inside the structs that
        // implement State, we call a content method on the value in state and
        // pass the post instance (that is, self) as an argument. Then we return
        // the value that is returned from using the content method on the state
        // value.

        // We call the as_ref method on the Option because we want a reference
        // to the value inside the Option rather than ownership of the value.
        // Because state is an Option<Box<dyn State>>, when we call as_ref, an
        // Option<&Box<dyn State>> is returned. If we didn’t call as_ref, we
        // would get an error because we can’t move state out of the borrowed
        // &self of the function parameter.

        // We then call the unwrap method, which we know will never panic,
        // because we know the methods on Post ensure that state will always
        // contain a Some value when those methods are done.

        // At this point, when we call content on the &Box<dyn State>, deref
        // coercion will take effect on the & and the Box so the content method
        // will ultimately be called on the type that implements the State
        // trait. That means we need to add content to the State trait
        // definition, and that is where we’ll put the logic for what content to
        // return depending on which state we have.
        self.state.as_ref().unwrap().content(self)
    }

    // Requesting a Review of the Post Changes Its State
    // Next, we need to add functionality to request a review of a post, which
    // should change its state from Draft to PendingReview.
    pub fn request_review(&mut self) {
        // We give Post a public method named request_review that will take a
        // mutable reference to self. Then we call an internal request_review
        // method on the current state of Post, and this second request_review
        // method consumes the current state and returns a new state.
        if let Some(s) = self.state.take() {
            // To consume the old state, the request_review method needs to take
            // ownership of the state value. This is where the Option in the
            // state field of Post comes in: we call the take method to take the
            // Some value out of the state field and leave a None in its place,
            // because Rust doesn’t let us have unpopulated fields in structs.
            // This lets us move the state value out of Post rather than
            // borrowing it. Then we’ll set the post’s state value to the result
            // of this operation.
            self.state = Some(s.request_review())
        }
    }

    // Adding the approve Method that Changes the Behavior of content
    // The approve method will be similar to the request_review method: it will
    // set state to the value that the current state says it should have when
    // that state is approved.
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// The State trait defines the behavior shared by different post states, and the
// Draft, PendingReview, and Published states will all implement the State
// trait. For now, the trait doesn’t have any methods, and we’ll start by
// defining just the Draft state because that is the state we want a post to
// start in.
trait State {
    // We’ve added the request_review method to the State trait; all types that
    // implement the trait will now need to implement the request_review method.
    // Note that rather than having self, &self, or &mut self as the first
    // parameter of the method, we have self: Box<Self>. This syntax means the
    // method is only valid when called on a Box holding the type. This syntax
    // takes ownership of Box<Self>, invalidating the old state so the state
    // value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // We add the approve method to the State trait and add a new struct that
    // implements State, the Published state.
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // We add a default implementation for the content method that returns an
    // empty string slice. That means we don’t need to implement content on the
    // Draft and PendingReview structs. The Published struct will override the
    // content method and return the value in post.content.
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    // The request_review method on Draft needs to return a new, boxed instance
    // of a new PendingReview struct, which represents the state when a post is
    // waiting for a review.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // Similar to the way request_review on PendingReview works, if we call the
    // approve method on a Draft, it will have no effect because approve will
    // return self.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    // The PendingReview struct also implements the request_review method but
    // doesn’t do any transformations. Rather, it returns itself, because when
    // we request a review on a post already in the PendingReview state, it
    // should stay in the PendingReview state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // When we call approve on PendingReview, it returns a new, boxed instance
    // of the Published struct.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // The Published struct implements the State trait, and for both the
    // request_review method and the approve method, it returns itself, because
    // the post should stay in the Published state in those cases.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // We’re taking a reference to a post as an argument and returning a
    // reference to part of that post, so the lifetime of the returned reference
    // is related to the lifetime of the post argument.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// We still enable the creation of new posts in the draft state using Post::new
// and the ability to add text to the post’s content. But instead of having a
// content method on a draft post that returns an empty string, we’ll make it so
// draft posts don’t have the content method at all. That way, if we try to get
// a draft post’s content, we’ll get a compiler error telling us the method
// doesn’t exist. As a result, it will be impossible for us to accidentally
// display draft post content in production, because that code won’t even
// compile.
pub struct Post2 {
    content: String,
}

// Both the Post and DraftPost structs have a private content field that stores
// the blog post text. The structs no longer have the state field because we’re
// moving the encoding of the state to the types of the structs. The Post struct
// will represent a published post, and it has a content method that returns the
// content.
pub struct DraftPost {
    content: String,
}

impl Post2 {
    // We still have a Post::new function, but instead of returning an instance
    // of Post, it returns an instance of DraftPost. Because content is private
    // and there aren’t any functions that return Post, it’s not possible to
    // create an instance of Post right now.
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // The DraftPost struct has an add_text method, so we can add text to
    // content as before, but note that DraftPost does not have a content method
    // defined! So now the program ensures all posts start as draft posts, and
    // draft posts don’t have their content available for display. Any attempt
    // to get around these constraints will result in a compiler error.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// Implementing Transitions as Transformations into Different Types
// So how do we get a published post? We want to enforce the rule that a draft
// post has to be reviewed and approved before it can be published. A post in
// the pending review state should still not display any content. Let’s
// implement these constraints by adding another struct, PendingReviewPost,
// defining the request_review method on DraftPost to return a
// PendingReviewPost, and defining an approve method on PendingReviewPost to
// return a Post.
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // The request_review and approve methods take ownership of self, thus
    // consuming the DraftPost and PendingReviewPost instances and transforming
    // them into a PendingReviewPost and a published Post, respectively. This
    // way, we won’t have any lingering DraftPost instances after we’ve called
    // request_review on them, and so forth. The PendingReviewPost struct
    // doesn’t have a content method defined on it, so attempting to read its
    // content results in a compiler error, as with DraftPost. Because the only
    // way to get a published Post instance that does have a content method
    // defined is to call the approve method on a PendingReviewPost, and the
    // only way to get a PendingReviewPost is to call the request_review method
    // on a DraftPost, we’ve now encoded the blog post workflow into the type
    // system.
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}
