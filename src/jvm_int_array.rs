use jni_sys::{ jintArray, jsize };
use jvm::Jvm;
use jvm_attachment::JvmAttachment;

jvm_array_wrapper!(JvmIntArray, jintArray);
