MAJOR.MINOR.PATCH

if 0.MINOR.PATCH
MINOR++ when ADD FEATURES
PATCH++ when BUGFIXES/MINOR CHANGE

0.1.0 (release)
- release

0.1.1
- filter add, force info

0.1.2 
- filter fold in gui
- projectile sim (template) add
- minor change

0.2.0 - Interaction
- add interaction(drag) to simulation object (current only force)
- refactor to simulation to under simulation_plot
- simulation can include math function y=f(x) or (x,y) = f(t)

0.2.1
- consistent coloring (visibility improve)
- plot legends add (visibility improve)

0.3.0 - Sim init
- now simulation can initialize!
- simulation can init by ui until it resumed.

0.3.1
- mobile drag fix
- inited simulation force add on every frame fix
- inited simulation view and interaction reset problem fix
- show fps
- now we can show inited simulation / base_init_sim
- dep log to tracing

0.4.0
- add simulation stamp (conditional simulation object marking)
- simulation main logic fixed (now Vec<Force> is changed by Vec<Velocity>)
- various bug fixes