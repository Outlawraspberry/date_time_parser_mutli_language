/*

Original License:

MIT License

Copyright (c) 2020 Isaac Lee and Alex Grimes

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Modifications:

- Add date format

Modified by Outlawraspberry UG (haftungsbeschränkt)
*/

use crate::language::shared::DateFormat;

/// An interface for dealing with parsing unstructured text. Implement this trait for your abstract syntax when parsing.
pub trait Recognizable: Sized {
    /// Takes unstructed text, and returns an instance of the abstract syntax if a match is found.
    fn recognize(text: &str, date_format: &DateFormat) -> Option<Self>;

    /// Returns a string to describe the abstract syntax.
    fn describe() -> &'static str;
}