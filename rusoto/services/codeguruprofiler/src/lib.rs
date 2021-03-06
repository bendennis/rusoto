// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p><p>This section provides documentation for the Amazon CodeGuru Profiler API operations.</p> <pre><code> &lt;p&gt;Amazon CodeGuru Profiler collects runtime performance data from your live applications, and provides recommendations that can help you fine-tune your application performance. Using machine learning algorithms, CodeGuru Profiler can help you find your most expensive lines of code and suggest ways you can improve efficiency and remove CPU bottlenecks. &lt;/p&gt; &lt;p&gt;Amazon CodeGuru Profiler provides different visualizations of profiling data to help you identify what code is running on the CPU, see how much time is consumed, and suggest ways to reduce CPU utilization. &lt;/p&gt; &lt;note&gt; &lt;p&gt;Amazon CodeGuru Profiler currently supports applications written in all Java virtual machine (JVM) languages. While CodeGuru Profiler supports both visualizations and recommendations for applications written in Java, it can also generate visualizations and a subset of recommendations for applications written in other JVM languages.&lt;/p&gt; &lt;/note&gt; &lt;p&gt; For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-ug/what-is-codeguru-profiler.html&quot;&gt;What is Amazon CodeGuru Profiler&lt;/a&gt; in the &lt;i&gt;Amazon CodeGuru Profiler User Guide&lt;/i&gt;. &lt;/p&gt; </code></pre></p>
//!
//! If you're using the service, you're probably looking for [CodeGuruProfilerClient](struct.CodeGuruProfilerClient.html) and [CodeGuruProfiler](trait.CodeGuruProfiler.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
