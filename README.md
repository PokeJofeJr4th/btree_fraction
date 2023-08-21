# Farey Tree Fractions

The data types included in this library are based on a Farey Tree, using an algorithm similar to a Farey Sequence to divide the positive rational number line into sections. The easiest way to understand this concept is to step through the approximation algorithm used to convert floats to fractions. Let's start approximating Euler's number, about 2.718.

First, we define the bounds of our range as zero (0/1) and infinity (1/0). The farey tree bisects any region of the number line by adding the numerator and denominator together, so the center of our initial range is 1 (1/1). The real Farey Tree only includes the space between 0 and 1, but extending to greater range is trivial.

At each step, we compare to the center number. `e` is greater than 1, so we replace the lower bound with 1 and set the center to `(1+1)/(0+1)`, or 2. `e` is greater than 2, so we replace the lower bound with 2 and set the center to `(2+1)/(0+1)`, or 3. This time, `e` is less than 3, so we set the upper bound to 3. Since our lower bound is still 2, the new center is `(2+3)/(1+1)`, or 5/2.

Here's a chart of the first few levels of the farey tree used for Farey Tree Fractions:

```
                              1/1
              1/2                             2/1
      1/3             2/3             3/2             3/1
  1/4     2/5     3/5     3/4     4/3     5/3     5/2     4/1
1/5 2/7 3/8 3/7 4/7 5/8 5/7 4/5 5/4 7/5 8/5 7/4 7/3 8/3 7/2 5/1
```
