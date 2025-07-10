const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.30.2-beta2");
#[allow(non_camel_case_types)]
pub struct Point {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for Point {}

impl ::std::default::Default for Point {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for Point {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for Point {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::MergeFrom for Point {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for Point {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for Point {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for Point {
  fn clear_and_parse(&mut self, data: &[u8]) -> ::std::result::Result<(), ::protobuf::ParseError> {
    let mut msg = Self::new();

    // SAFETY:
    // - `data.as_ptr()` is valid to read for `data.len()`
    // - `mini_table` is the one used to construct `msg.raw_msg()`
    // - `msg.arena().raw()` is held for the same lifetime as `msg`.
    let status = unsafe {
      ::protobuf::__internal::runtime::wire::decode(
          data,
          msg.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          msg.arena())
    };
    match status {
      Ok(_) => {
        ::std::mem::swap(self, &mut msg);
        Ok(())
      }
      Err(_) => Err(::protobuf::ParseError)
    }
  }
}

// SAFETY:
// - `Point` is `Sync` because it does not implement interior mutability.
//    Neither does `PointMut`.
unsafe impl Sync for Point {}

// SAFETY:
// - `Point` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Point {}

impl ::protobuf::Proxied for Point {
  type View<'msg> = PointView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Point {}

impl ::protobuf::MutProxied for Point {
  type Mut<'msg> = PointMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PointView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PointView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PointView<'msg> {
  type Message = Point;
}

impl ::std::fmt::Debug for PointView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PointView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PointView<'_> {
  fn default() -> PointView<'static> {
    PointView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::ScratchSpace::zeroed_block())
  }
}

#[allow(dead_code)]
impl<'msg> PointView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> Point {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // latitude: optional int32
  pub fn latitude(self) -> i32 {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::protobuf::__internal::runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (0i32).into()).try_into().unwrap()
    }
  }

  // longitude: optional int32
  pub fn longitude(self) -> i32 {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 1);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::protobuf::__internal::runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (0i32).into()).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PointView` is `Sync` because it does not support mutation.
unsafe impl Sync for PointView<'_> {}

// SAFETY:
// - `PointView` is `Send` because while its alive a `PointMut` cannot.
// - `PointView` does not use thread-local data.
unsafe impl Send for PointView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PointView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PointView<'msg> {}

impl<'msg> ::protobuf::AsView for PointView<'msg> {
  type Proxied = Point;
  fn as_view(&self) -> ::protobuf::View<'msg, Point> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PointView<'msg> {
  fn into_view<'shorter>(self) -> PointView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Point> for PointView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Point {
    let dst = Point::new();
    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst.inner.msg,
      self.msg,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst.inner.arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Point> for PointMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Point {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for Point {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    let arena = ::protobuf::__internal::runtime::Arena::new();
    unsafe {
      ::protobuf::Repeated::from_inner(
          ::protobuf::__internal::Private,
          ::protobuf::__internal::runtime::InnerRepeated::from_raw_parts(
              ::protobuf::__internal::runtime::upb_Array_New(arena.raw(), ::protobuf::__internal::runtime::CType::Message),
              arena,
          ))
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, _f: &mut ::protobuf::Repeated<Self>) {
    // No-op: the memory will be dropped by the arena.
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `upb_Array*`.
    unsafe { ::protobuf::__internal::runtime::upb_Array_Size(f.as_raw(::protobuf::__internal::Private)) }
  }
  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    unsafe {
        ::protobuf::__internal::runtime::upb_Array_Set(
            f.as_raw(::protobuf::__internal::Private),
            i,
            <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                f.raw_arena(::protobuf::__internal::Private),
                v.into_proxied(::protobuf::__internal::Private),
            ),
        )
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const upb_Array*`.
    // - `i < len(f)` is promised by the caller.
    let msg_ptr = unsafe { ::protobuf::__internal::runtime::upb_Array_Get(f.as_raw(::protobuf::__internal::Private), i).msg_val }
      .expect("upb_Array* element should not be NULL.");
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg_ptr)
  }

  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    unsafe {
      ::protobuf::__internal::runtime::upb_Array_Resize(f.as_raw(::protobuf::__internal::Private), 0, f.raw_arena(::protobuf::__internal::Private))
    };
  }
  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    // - `msg_ptr` is a valid `upb_Message*`.
    unsafe {
      ::protobuf::__internal::runtime::upb_Array_Append(
        f.as_raw(::protobuf::__internal::Private),
        <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(f.raw_arena(::protobuf::__internal::Private), v.into_proxied(::protobuf::__internal::Private)),
        f.raw_arena(::protobuf::__internal::Private)
      );
    };
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
      // SAFETY:
      // - Elements of `src` and `dest` have message minitable `MINI_TABLE`.
      unsafe {
        ::protobuf::__internal::runtime::repeated_message_copy_from(src, dest, <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table());
      }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    unsafe {
      let size = ::protobuf::__internal::runtime::upb_Array_Size(f.as_raw(::protobuf::__internal::Private));
      ::protobuf::__internal::runtime::upb_Array_Reserve(f.as_raw(::protobuf::__internal::Private), size + additional, f.raw_arena(::protobuf::__internal::Private));
    }
  }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for Point {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_mutator_message_ref(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        PointView::new(
            ::protobuf::__internal::Private,
            unsafe { msg.msg_val }
                .expect("expected present message value in map"))
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PointMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PointMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PointMut<'msg> {
  type Message = Point;
}

impl ::std::fmt::Debug for PointMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PointMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for PointMut<'_> {
  fn clear(&mut self) {
    unsafe {
      ::protobuf::__internal::runtime::upb_Message_Clear(
          self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    }
  }
}

impl ::protobuf::MergeFrom for PointMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = Point>) {
    // SAFETY: self and src are both valid `Point`s.
    unsafe {
      assert!(
        ::protobuf::__internal::runtime::upb_Message_MergeFrom(self.raw_msg(),
          src.as_view().raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          // Use a nullptr for the ExtensionRegistry.
          ::std::ptr::null(),
          self.arena().raw())
      );
    }
  }
}

#[allow(dead_code)]
impl<'msg> PointMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: &'msg mut ::protobuf::__internal::runtime::MessageInner) -> Self {
    Self{ inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg) }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg()
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn to_owned(&self) -> Point {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // latitude: optional int32
  pub fn latitude(&self) -> i32 {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::protobuf::__internal::runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (0i32).into()).try_into().unwrap()
    }
  }
  pub fn set_latitude(&mut self, val: i32) {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly.
      ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
          self.raw_msg(), f, val.into());
    }
  }

  // longitude: optional int32
  pub fn longitude(&self) -> i32 {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 1);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::protobuf::__internal::runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (0i32).into()).try_into().unwrap()
    }
  }
  pub fn set_longitude(&mut self, val: i32) {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 1);
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly.
      ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
          self.raw_msg(), f, val.into());
    }
  }

}

// SAFETY:
// - `PointMut` does not perform any shared mutation.
// - `PointMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PointMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PointMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PointMut<'msg> {}

impl<'msg> ::protobuf::AsView for PointMut<'msg> {
  type Proxied = Point;
  fn as_view(&self) -> ::protobuf::View<'_, Point> {
    PointView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PointMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Point>
  where
      'msg: 'shorter {
    PointView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for PointMut<'msg> {
  type MutProxied = Point;
  fn as_mut(&mut self) -> PointMut<'msg> {
    PointMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PointMut<'msg> {
  fn into_mut<'shorter>(self) -> PointMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Point {
  pub fn new() -> Self {
    let arena = ::protobuf::__internal::runtime::Arena::new();
    let raw_msg = unsafe {
        ::protobuf::__internal::runtime::upb_Message_New(
            <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            arena.raw()).unwrap()
    };
    Self {
      inner: ::protobuf::__internal::runtime::MessageInner {
        msg: raw_msg,
        arena,
      }
    }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MutatorMessageRef {
    ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
  }

  fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
    &self.inner.arena
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PointView {
    PointView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> PointMut {
    PointMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // latitude: optional int32
  pub fn latitude(&self) -> i32 {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::protobuf::__internal::runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (0i32).into()).try_into().unwrap()
    }
  }
  pub fn set_latitude(&mut self, val: i32) {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly.
      ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
          self.raw_msg(), f, val.into());
    }
  }

  // longitude: optional int32
  pub fn longitude(&self) -> i32 {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 1);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::protobuf::__internal::runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (0i32).into()).try_into().unwrap()
    }
  }
  pub fn set_longitude(&mut self, val: i32) {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 1);
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly.
      ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
          self.raw_msg(), f, val.into());
    }
  }

}  // impl Point

impl ::std::ops::Drop for Point {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Point {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Point {
  type Proxied = Self;
  fn as_view(&self) -> PointView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Point {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PointMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Point {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::std::ptr::addr_of!(routeguide__Point_msg_init)
    }
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PointView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::std::ptr::addr_of!(routeguide__Point_msg_init)
    }
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PointMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::std::ptr::addr_of!(routeguide__Point_msg_init)
    }
  }
}

extern "C" {
  /// Opaque static extern for this message's MiniTable, generated
  /// by the upb C MiniTable codegen. The only valid way to
  /// reference this static is with `std::ptr::addr_of!(..)`.
  static routeguide__Point_msg_init: ::protobuf::__internal::runtime::upb_MiniTable;
}

// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for Point {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PointMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PointView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::__internal::MatcherEq for Point {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for PointMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for PointView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::upb_Message_IsEqual(
          self.msg,
          o.msg,
          <Point as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0)
    }
  }
}

#[allow(non_camel_case_types)]
pub struct Feature {
  inner: ::protobuf::__internal::runtime::MessageInner
}

impl ::protobuf::Message for Feature {}

impl ::std::default::Default for Feature {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for Feature {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }
}

impl ::std::fmt::Debug for Feature {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::MergeFrom for Feature {
  fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::protobuf::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::protobuf::Serialize for Feature {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for Feature {
  fn clear(&mut self) {
    let mut m = self.as_mut();
    ::protobuf::Clear::clear(&mut m)
  }
}

impl ::protobuf::ClearAndParse for Feature {
  fn clear_and_parse(&mut self, data: &[u8]) -> ::std::result::Result<(), ::protobuf::ParseError> {
    let mut msg = Self::new();

    // SAFETY:
    // - `data.as_ptr()` is valid to read for `data.len()`
    // - `mini_table` is the one used to construct `msg.raw_msg()`
    // - `msg.arena().raw()` is held for the same lifetime as `msg`.
    let status = unsafe {
      ::protobuf::__internal::runtime::wire::decode(
          data,
          msg.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          msg.arena())
    };
    match status {
      Ok(_) => {
        ::std::mem::swap(self, &mut msg);
        Ok(())
      }
      Err(_) => Err(::protobuf::ParseError)
    }
  }
}

// SAFETY:
// - `Feature` is `Sync` because it does not implement interior mutability.
//    Neither does `FeatureMut`.
unsafe impl Sync for Feature {}

// SAFETY:
// - `Feature` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Feature {}

impl ::protobuf::Proxied for Feature {
  type View<'msg> = FeatureView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Feature {}

impl ::protobuf::MutProxied for Feature {
  type Mut<'msg> = FeatureMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FeatureView<'msg> {
  msg: ::protobuf::__internal::runtime::RawMessage,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FeatureView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FeatureView<'msg> {
  type Message = Feature;
}

impl ::std::fmt::Debug for FeatureView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for FeatureView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for FeatureView<'_> {
  fn default() -> FeatureView<'static> {
    FeatureView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::ScratchSpace::zeroed_block())
  }
}

#[allow(dead_code)]
impl<'msg> FeatureView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: ::protobuf::__internal::runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> Feature {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // location: optional message routeguide.Point
  pub fn has_location(self) -> bool {
    unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          1);
      ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
    }
  }
  pub fn location_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
  }
  pub fn location(self) -> super::PointView<'msg> {
    let submsg = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                  <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                  1);
      ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
    };
    match submsg {
      None => super::PointView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::ScratchSpace::zeroed_block()),
      Some(sub_raw_msg) => super::PointView::new(::protobuf::__internal::Private, sub_raw_msg),
    }
  }

}

// SAFETY:
// - `FeatureView` is `Sync` because it does not support mutation.
unsafe impl Sync for FeatureView<'_> {}

// SAFETY:
// - `FeatureView` is `Send` because while its alive a `FeatureMut` cannot.
// - `FeatureView` does not use thread-local data.
unsafe impl Send for FeatureView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for FeatureView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for FeatureView<'msg> {}

impl<'msg> ::protobuf::AsView for FeatureView<'msg> {
  type Proxied = Feature;
  fn as_view(&self) -> ::protobuf::View<'msg, Feature> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FeatureView<'msg> {
  fn into_view<'shorter>(self) -> FeatureView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Feature> for FeatureView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Feature {
    let dst = Feature::new();
    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst.inner.msg,
      self.msg,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst.inner.arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Feature> for FeatureMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Feature {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for Feature {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    let arena = ::protobuf::__internal::runtime::Arena::new();
    unsafe {
      ::protobuf::Repeated::from_inner(
          ::protobuf::__internal::Private,
          ::protobuf::__internal::runtime::InnerRepeated::from_raw_parts(
              ::protobuf::__internal::runtime::upb_Array_New(arena.raw(), ::protobuf::__internal::runtime::CType::Message),
              arena,
          ))
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, _f: &mut ::protobuf::Repeated<Self>) {
    // No-op: the memory will be dropped by the arena.
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `upb_Array*`.
    unsafe { ::protobuf::__internal::runtime::upb_Array_Size(f.as_raw(::protobuf::__internal::Private)) }
  }
  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    unsafe {
        ::protobuf::__internal::runtime::upb_Array_Set(
            f.as_raw(::protobuf::__internal::Private),
            i,
            <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                f.raw_arena(::protobuf::__internal::Private),
                v.into_proxied(::protobuf::__internal::Private),
            ),
        )
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const upb_Array*`.
    // - `i < len(f)` is promised by the caller.
    let msg_ptr = unsafe { ::protobuf::__internal::runtime::upb_Array_Get(f.as_raw(::protobuf::__internal::Private), i).msg_val }
      .expect("upb_Array* element should not be NULL.");
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg_ptr)
  }

  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    unsafe {
      ::protobuf::__internal::runtime::upb_Array_Resize(f.as_raw(::protobuf::__internal::Private), 0, f.raw_arena(::protobuf::__internal::Private))
    };
  }
  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    // - `msg_ptr` is a valid `upb_Message*`.
    unsafe {
      ::protobuf::__internal::runtime::upb_Array_Append(
        f.as_raw(::protobuf::__internal::Private),
        <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(f.raw_arena(::protobuf::__internal::Private), v.into_proxied(::protobuf::__internal::Private)),
        f.raw_arena(::protobuf::__internal::Private)
      );
    };
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
      // SAFETY:
      // - Elements of `src` and `dest` have message minitable `MINI_TABLE`.
      unsafe {
        ::protobuf::__internal::runtime::repeated_message_copy_from(src, dest, <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table());
      }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    unsafe {
      let size = ::protobuf::__internal::runtime::upb_Array_Size(f.as_raw(::protobuf::__internal::Private));
      ::protobuf::__internal::runtime::upb_Array_Reserve(f.as_raw(::protobuf::__internal::Private), size + additional, f.raw_arena(::protobuf::__internal::Private));
    }
  }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for Feature {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_mutator_message_ref(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        FeatureView::new(
            ::protobuf::__internal::Private,
            unsafe { msg.msg_val }
                .expect("expected present message value in map"))
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FeatureMut<'msg> {
  inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FeatureMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FeatureMut<'msg> {
  type Message = Feature;
}

impl ::std::fmt::Debug for FeatureMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for FeatureMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

impl ::protobuf::Clear for FeatureMut<'_> {
  fn clear(&mut self) {
    unsafe {
      ::protobuf::__internal::runtime::upb_Message_Clear(
          self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    }
  }
}

impl ::protobuf::MergeFrom for FeatureMut<'_> {
  fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = Feature>) {
    // SAFETY: self and src are both valid `Feature`s.
    unsafe {
      assert!(
        ::protobuf::__internal::runtime::upb_Message_MergeFrom(self.raw_msg(),
          src.as_view().raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          // Use a nullptr for the ExtensionRegistry.
          ::std::ptr::null(),
          self.arena().raw())
      );
    }
  }
}

#[allow(dead_code)]
impl<'msg> FeatureMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, msg: &'msg mut ::protobuf::__internal::runtime::MessageInner) -> Self {
    Self{ inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg) }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg()
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn to_owned(&self) -> Feature {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let mm_ref =
      self.as_mutator_message_ref(::protobuf::__internal::Private);
    let parent_arena = mm_ref.arena();

    parent_arena.fuse(&arena);

    unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0);
      ::protobuf::__internal::runtime::upb_Message_SetBaseFieldString(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        f,
        view);
    }
  }

  // location: optional message routeguide.Point
  pub fn has_location(&self) -> bool {
    unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          1);
      ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
    }
  }
  pub fn clear_location(&mut self) {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 1);
      ::protobuf::__internal::runtime::upb_Message_ClearBaseField(self.raw_msg(), f);
    }
  }
  pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
  }
  pub fn location(&self) -> super::PointView<'_> {
    let submsg = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                  <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                  1);
      ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
    };
    match submsg {
      None => super::PointView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::ScratchSpace::zeroed_block()),
      Some(sub_raw_msg) => super::PointView::new(::protobuf::__internal::Private, sub_raw_msg),
    }
  }
  pub fn location_mut(&mut self) -> super::PointMut<'_> {
     let raw_msg = unsafe {
       let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
       let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(mt, 1);
       ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
           self.raw_msg(), mt, f, self.arena().raw()).unwrap()
     };
     super::PointMut::from_parent(::protobuf::__internal::Private,
       self.as_mutator_message_ref(::protobuf::__internal::Private), raw_msg)
  }
  pub fn set_location(&mut self,
    val: impl ::protobuf::IntoProxied<super::Point>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut msg = val.into_proxied(::protobuf::__internal::Private);
    self.as_mutator_message_ref(::protobuf::__internal::Private)
      .arena()
      .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());

    unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1);
      ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        f,
        msg.as_mutator_message_ref(::protobuf::__internal::Private).msg());
    }
  }

}

// SAFETY:
// - `FeatureMut` does not perform any shared mutation.
// - `FeatureMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for FeatureMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for FeatureMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for FeatureMut<'msg> {}

impl<'msg> ::protobuf::AsView for FeatureMut<'msg> {
  type Proxied = Feature;
  fn as_view(&self) -> ::protobuf::View<'_, Feature> {
    FeatureView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FeatureMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Feature>
  where
      'msg: 'shorter {
    FeatureView { msg: self.raw_msg(), _phantom: ::std::marker::PhantomData }
  }
}

impl<'msg> ::protobuf::AsMut for FeatureMut<'msg> {
  type MutProxied = Feature;
  fn as_mut(&mut self) -> FeatureMut<'msg> {
    FeatureMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FeatureMut<'msg> {
  fn into_mut<'shorter>(self) -> FeatureMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Feature {
  pub fn new() -> Self {
    let arena = ::protobuf::__internal::runtime::Arena::new();
    let raw_msg = unsafe {
        ::protobuf::__internal::runtime::upb_Message_New(
            <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            arena.raw()).unwrap()
    };
    Self {
      inner: ::protobuf::__internal::runtime::MessageInner {
        msg: raw_msg,
        arena,
      }
    }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.msg
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MutatorMessageRef {
    ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
  }

  fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
    &self.inner.arena
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> FeatureView {
    FeatureView::new(::protobuf::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> FeatureMut {
    FeatureMut::new(::protobuf::__internal::Private, &mut self.inner)
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let mm_ref =
      self.as_mutator_message_ref(::protobuf::__internal::Private);
    let parent_arena = mm_ref.arena();

    parent_arena.fuse(&arena);

    unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0);
      ::protobuf::__internal::runtime::upb_Message_SetBaseFieldString(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        f,
        view);
    }
  }

  // location: optional message routeguide.Point
  pub fn has_location(&self) -> bool {
    unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          1);
      ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
    }
  }
  pub fn clear_location(&mut self) {
    unsafe {
      let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          mt, 1);
      ::protobuf::__internal::runtime::upb_Message_ClearBaseField(self.raw_msg(), f);
    }
  }
  pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
  }
  pub fn location(&self) -> super::PointView<'_> {
    let submsg = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                  <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                  1);
      ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
    };
    match submsg {
      None => super::PointView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::ScratchSpace::zeroed_block()),
      Some(sub_raw_msg) => super::PointView::new(::protobuf::__internal::Private, sub_raw_msg),
    }
  }
  pub fn location_mut(&mut self) -> super::PointMut<'_> {
     let raw_msg = unsafe {
       let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
       let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(mt, 1);
       ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
           self.raw_msg(), mt, f, self.arena().raw()).unwrap()
     };
     super::PointMut::from_parent(::protobuf::__internal::Private,
       self.as_mutator_message_ref(::protobuf::__internal::Private), raw_msg)
  }
  pub fn set_location(&mut self,
    val: impl ::protobuf::IntoProxied<super::Point>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut msg = val.into_proxied(::protobuf::__internal::Private);
    self.as_mutator_message_ref(::protobuf::__internal::Private)
      .arena()
      .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());

    unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1);
      ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
        self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
        f,
        msg.as_mutator_message_ref(::protobuf::__internal::Private).msg());
    }
  }

}  // impl Feature

impl ::std::ops::Drop for Feature {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Feature {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Feature {
  type Proxied = Self;
  fn as_view(&self) -> FeatureView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Feature {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FeatureMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Feature {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::std::ptr::addr_of!(routeguide__Feature_msg_init)
    }
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeatureView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::std::ptr::addr_of!(routeguide__Feature_msg_init)
    }
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeatureMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::std::ptr::addr_of!(routeguide__Feature_msg_init)
    }
  }
}

extern "C" {
  /// Opaque static extern for this message's MiniTable, generated
  /// by the upb C MiniTable codegen. The only valid way to
  /// reference this static is with `std::ptr::addr_of!(..)`.
  static routeguide__Feature_msg_init: ::protobuf::__internal::runtime::upb_MiniTable;
}

// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for Feature {}
impl<'a> ::protobuf::MessageMutInterop<'a> for FeatureMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for FeatureView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    Self::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap())
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::protobuf::__internal::MatcherEq for Feature {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for FeatureMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::protobuf::__internal::MatcherEq::matches(
      &::protobuf::AsView::as_view(self),
      &::protobuf::AsView::as_view(o))
  }
}

impl<'a> ::protobuf::__internal::MatcherEq for FeatureView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::protobuf::__internal::runtime::upb_Message_IsEqual(
          self.msg,
          o.msg,
          <Feature as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0)
    }
  }
}

