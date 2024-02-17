# Calculadora de Vacaciones
Sistema que calcula la cantidad de días de vacaciones disponibles para empleados Chilenos contratados.

## Modo de Uso
1) Se debe modificar el archivo [data.json](/docs/data.json) con la información del empleado.
    - `nombre`: Nombre del empleado.
    - `fecha_ingreso`: Fecha de contratación en su empresa.
    - `dias_utilizados`: Días de vacaciones que a utilizado hasta la fecha.
2) Ejecutar el código mediante cargo: `cargo run --release`.
3) El sistema desplegará cuantos días de vacaciones el empleado tiene acumulado hasta la fecha.
4) Luego, el sistema pregunta si el usuario desea recalcular asumiendo que se toma una cierta cantidad de días.
    - Debe ingresa `s` para proceder, o `n` para terminar el programa.
5) El sistema solicitará ingresar la cantidad de días a tomarse, tras lo cuál recalcula el total de días disponibles.
6) Finalmente, el programa pregunta al usuario si desea modificar el archivo con la cantidad de días de vacaciones ingresados.
    - Debe ingresa `s` para proceder, tras lo cuál se modificará el archivo [data.json](/docs/data.json) automáticamente, o `n` para terminar el programa.