TODO: make this pretty

[TOC]

# Non-ideal resistor

## Frequency Response

A real resistor has parasitic capacitance and inductance, so it doesn't have a flat frequency response, however this is negligible at audio rate.

## Thermal Drift

$R_t=R_o[1+\alpha(T-T_o)]$

Where:

- $R_o$ is the nominal resistance
- $\alpha$ is the resistor's thermal coefficient
- $T$ is the ambient temperature in Kelvin or Celsius
- $T_o$ is the nominal temperature in Kelvin or Celsius (temperature at which $R_o$ is taken)

LUT

| Type        | $R_o$             | $\alpha$                                 | $T_o$ |
| ----------- | ----------------- | ---------------------------------------- | ----- |
| carbon film | $<10\Omega$       | from $0$ to $-200E-6{\quad^\circ}C^{-1}$ | 70 C  |
| carbon film | up to $99k\Omega$ | up to $-450E-6{\quad^\circ}C^{-1}$       | 70 C  |
| carbon film | up to $1M\Omega$  | up to $-700E-6{\quad^\circ}C^{-1}$       | 70 C  |
| carbon film | up to $10M\Omega$ | up to $-1500E-6\quad^\circ C^{-1}$       | 70 C  |
| metal film  | < $1 M\Omega$     | from 0 to $-50E-6\quad^\circ C^{-1}$     | 70 C  |

[Carbon Film Datasheet](http://www.farnell.com/datasheets/1716725.pdf?_ga=2.43716581.132370299.1498698481-246720399.1496759099)

[Metal Film Datasheet](https://datasheet.octopart.com/MF25-1R-Multicomp-datasheet-13709972.pdf)

## Thermal Noise

Thermal noise is white noise with RMS voltage:

$E_{RMS}\approx 7.4314158E-12\sqrt{R\cdot T\cdot\Delta f}$

Where

* $T$ is the ambient temperature in Kelvin
* $\Delta f$ is the bandwidth of the signal (Nyquist for digitized model)

For generating noise, we can assume that the variance $\sigma^2=E_{RMS}$, then for uniform distribution:

$\sigma^2=\dfrac{range^2}{12}\implies range=\sqrt{E_{RMS}\cdot 12}$

Meaning the amplitude of the noise in voltage will be:

$A=\sqrt{(8.91769896E-11)\cdot\sqrt{RT\Delta f}}$

The total voltage out will be:

$V_{out}=V_{in}-RI+\sqrt{(8.91769896E-11)\cdot\sqrt{RT\Delta f}}$

Solving for $R$ we get:

Vout = Vin - Vt + Vnoise

Vout - Vin = Vnoise - Vt

Vin - Vout = Vt - Vnoise

deltaV = Vt - Vnoise

R=Rt - Rnoise = Rt - A/I = Rt - A/(Vt/R)

## Flicker Noise

The flicker noise is pink noise of unknown origin that has this voltage spectral distribution:

$S(f)=\dfrac{CV^2}{f}$

Where

* $C$ is the Hooge constant of the resistor
* $V$ is the voltage drop across the resistor

Note that it is dependent on the voltage, so it's modulated by the input signal.

[Source](https://core.ac.uk/download/pdf/30277457.pdf)

$C\approx 3E-13$ for carbon film and $2E-16$ for metal film.

Note that the spectral density can be written as:

$S(f)=\dfrac{1}{f}\cdot CV^2$

Where 1/f is full-gain pink noise, scaled by a constant $CV^2$, so we can integrate the $1/f$ term alone over the bandwidth of the signal, and the constant would not change under integration, so the formula for the noise is:

$V_{out}=(V_{in}-V_R)+pink\_noise\times CV_R^2$

## Putting it together

Putting the equations for noise and thermal drift together, we get:

$R_t=R_o[1+\alpha(T-T_o)]$

$V_{noise}=white\_noise\sqrt{(8.91769896E-11)\cdot\sqrt{R_tT\Delta f}}+pink\_noise\cdot CV_R^2$

$V_{out}=V_{in}-V_R+V_{noise}$

$V_{in}-V_{out} = V_{noise} + V_t$

$\Delta V=V_{noise}+V_t$

$R=\dfrac{V_{noise}}{I}+R_t$

I cannot be known as we need to know the rest of the circuit, but we can approximate it if we assume that each component works on unity gain signals, so that the voltage drop across every component is always 1V

$I=\Delta V/R\approx 1/R$

If we set that in we get:

$R\approx R\cdot V_{noise} + R_t$

We would like to remove the circular dependence between $R$ and itself, so we can approximate by realizing that the noise component will never have a large effect on the resistance, and thus we can substitute $R_t$ for $R$:

$R\approx R_t V_{noise}+R_t=R_t(V_{noise}+1)$

Now we can substitute all the rest of the equations in, taking care to set $V_R$ in the formula for 1/f noise to 1V:

$\begin{cases}R_t=R_o[1+\alpha(T-T_o)]\\R\approx R_t\cdot(white\_noise\sqrt{(8.91769896E-11)\cdot\sqrt{R_tT\Delta f}}+pink\_noise\cdot C+1)\end{cases}$

This gives a good implementation of an analog resistor, we can define separate functions for carbon film and metal film resistors for convenience:

### Carbon Film Resistance Function

Assuming $T_0=70$

$C=3E-13$

$R_t = R_0[1 + LUT(R_0, \text{"carbon"}) (T- 70)]$

$R(R_0, T, f_s)=R_t\cdot(white\_noise\sqrt{(8.91769896E-11)\cdot\sqrt{R_tTf_s}}+pink\_noise\cdot (3E-13)+1)$

### Metal Film Resistance Function

Assuming $T_0=70$

$C=2E-16$

$R_t = R_0[1 + LUT(R_0, \text{"metal"}) (T- 70)]$

$R(R_0, T, f_s)=R_t\cdot(white\_noise\sqrt{(8.91769896E-11)\cdot\sqrt{R_tTf_s}}+pink\_noise\cdot (2E-16)+1)$

# Non-ideal capacitor



