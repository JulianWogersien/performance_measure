# performance_measure
measures performance of some code in rust

To use first create a Measurer with `Measurer::new(Option::None)`
the option here gives you a way to control the number of samples it will keep the default is 1000
then you can simply start measuring performance by either calling measure_closure on the measurer variable
which will return the average time it took to execute said closure
or you can call start_measure and then execute whatever code you want to measure 
and then after the code you want to measure either call stop_measure which will add a sample if you havent reached the maximum samples and otherwise do nothing
if you want to instead replace old samples once you reached the maximum samples you want to instead use the stop_measure_replace_old function
this approach is more for if you have some sort of loop whereas the closure approach is just generally any sort of performance measuring of some piece of code
at any point you can get the average the min the max the median the variance the standard deviation and the mode or the samples by calling the appropiate function

