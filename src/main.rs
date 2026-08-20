use yew::prelude::*;
use num_integer::gcd;

#[component]
fn Calculator() -> Html {
    let current_number = use_state(|| 0);
    let ticker = use_state(|| 0);
    let first_number = use_state(|| 0);
    let second_number = use_state(|| 0);
    let operator = use_state(|| 0);
    let display = use_state(|| String::from(""));
    let current_number_numerator = use_state(|| 0);
    let current_number_denominator = use_state(|| 0);
    let fraction = use_state(|| 0);

    let equals = {
        let second_number = second_number.clone();
        let first_number = first_number.clone();
        let current_number = current_number.clone();
        let operator = operator.clone();
        let ticker = ticker.clone();
        let display = display.clone();
        let current_number_numerator = current_number_numerator.clone();
        let current_number_denominator = current_number_denominator.clone();
        let fraction = fraction.clone();
        Callback::from(move |_: MouseEvent| {
            if *ticker == 1 {
            ticker.set(2);
            second_number.set(*current_number);
            if *fraction == 0 {
            current_number_numerator.set(*first_number);
            current_number_denominator.set(1)
            }
            if *operator == 1 {
            let greatestcommondivisor: i32 = gcd(*current_number_numerator + (*second_number * *current_number_denominator), *current_number_denominator);
            if *fraction == 1 {
            current_number.set(0);
            fraction.set(1);
            current_number_numerator.set((*current_number_numerator + (*second_number * *current_number_denominator))/ greatestcommondivisor);
            current_number_denominator.set(*current_number_denominator / greatestcommondivisor);
            display.set({*current_number_numerator}.to_string() + "/" + &{*current_number_denominator}.to_string());
            }
            else {
            current_number.set(*first_number + *second_number);
            display.set({*current_number}.to_string());
            fraction.set(0)
            }
            }
            else if *operator == 2 {
            let greatestcommondivisor: i32 = gcd(*current_number_numerator - (*second_number * *current_number_denominator), *current_number_denominator);
            if *fraction == 1 {
            current_number.set(0);
            fraction.set(1);
            current_number_numerator.set((*current_number_numerator - (*second_number * *current_number_denominator))/ greatestcommondivisor);
            current_number_denominator.set(*current_number_denominator / greatestcommondivisor);
            display.set({*current_number_numerator}.to_string() + "/" + &{*current_number_denominator}.to_string());
            }
            else {
            current_number.set(*first_number - *second_number);
            display.set({*current_number}.to_string());
            fraction.set(0)
            }
            }
            else if *operator == 3 {
            let greatestcommondivisor: i32 = gcd(*current_number_numerator * *second_number, *current_number_denominator);
            if *current_number_denominator/greatestcommondivisor == 1 {
            current_number.set((*current_number_numerator * *second_number) / greatestcommondivisor);
            display.set({*current_number}.to_string());
            fraction.set(0)
            }
            else if *current_number_denominator/greatestcommondivisor > 1 {
                current_number_numerator.set((*current_number_numerator * *second_number) / greatestcommondivisor);
                current_number_denominator.set(*current_number_denominator / greatestcommondivisor);
                fraction.set(1);
                display.set({*current_number_numerator}.to_string() + "/" + &{*current_number_denominator}.to_string());
            }
            }
            else if *operator == 4 {
            let greatestcommondivisor: i32 = gcd(*current_number_numerator, *second_number * *current_number_denominator);
            if (*second_number * *current_number_denominator)/greatestcommondivisor > 1{
            current_number_numerator.set(*current_number_numerator/greatestcommondivisor);
            current_number_denominator.set(*second_number * *current_number_denominator/greatestcommondivisor);
            fraction.set(1);
            display.set({*current_number_numerator}.to_string() + "/" + &{*current_number_denominator}.to_string());
            }
            else if {(*second_number * *current_number_denominator)/greatestcommondivisor} == 1 {
            current_number.set(*current_number_numerator/greatestcommondivisor);
            display.set({*current_number}.to_string());
            fraction.set(0);
            }
            else if *second_number == 0 {
                display.set("error".to_string());
                current_number.set(0);
                current_number_denominator.set(0);
                current_number_numerator.set(0);
                fraction.set(0);
            }
        }   
            second_number.set(0);
            first_number.set(0);
            ticker.set(0);
        }
        else{}
    })
    };
    
        let clear = {
        let second_number = second_number.clone();
        let first_number = first_number.clone();
        let current_number = current_number.clone();
        let operator = operator.clone();
        let ticker = ticker.clone();
        let display = display.clone();
        let current_number_numerator = current_number_numerator.clone();
        let current_number_denominator = current_number_denominator.clone();
        let fraction = fraction.clone();
        Callback::from(move |_: MouseEvent| {
            ticker.set(0);
            second_number.set(0);
            first_number.set(0);
            current_number.set(0);
            current_number_denominator.set(0);
            current_number_numerator.set(0);
            fraction.set(0);
            operator.set(0);
            display.set(String::from(""));
    })
    };

    let operator_update = |operator_type: i32|{
        let operator = operator.clone();
        let first_number = first_number.clone();
        let current_number = current_number.clone();
        let ticker = ticker.clone();
        Callback::from(move |_: MouseEvent| {
            if *ticker == 0 {
            ticker.set(1);
            operator.set(operator_type);
            first_number.set(*current_number);
            current_number.set(0);
        }
        else{}
    })
    };

        let digit_update = |digit: i32| {
        let current_number = current_number.clone();
        let display = display.clone();
        Callback::from(move |_: MouseEvent|  {
            let value = *current_number *10 + digit;
            current_number.set(value);
            display.set({*current_number}.to_string());
        })
    };

    html! {
        <div style = "display: flex; justify-content: center; align-items: center;">
        <div style = "background-color: gray; display: flex; justify-content: center; align-items: center; width: 330px; height: 410px; border: 3px solid black;"> <p>
        <div style = "display: flex; flex-direction: column; gap: 10px;">
        <div style="display: flex; justify-content: center; gap: 10px;">
            <button style="background-color: lightgray; width: 310px; height: 70px; font-size: 50px; text-align: right; border: 2px solid black;">{ <String as Clone>::clone(&*display) }</button>
        </div>
        <div style="display: flex; justify-content: center; gap: 10px;">
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(7)}>{ "7" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(8)}>{ "8" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(9)}>{"9"}</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={operator_update(4)}>{ "/" }</button>
        </div>
        <div style="display: flex; justify-content: center; gap: 10px;">
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(4)}>{ "4" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(5)}>{ "5" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(6)}>{ "6" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={operator_update(3)}>{ "*" }</button>
        </div>
        <div style="display: flex; justify-content: center; gap: 10px;">
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(1)}>{ "1" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(2)}>{ "2" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(3)}>{ "3" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={operator_update(2)}>{ "-" }</button>
        </div>
        <div style="display: flex; justify-content: center; gap: 10px;">
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={clear}>{ "C" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={digit_update(0)}>{ "0" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={equals}>{ "=" }</button>
            <button style="width: 70px; height: 70px; font-size: 50px; border: 2px solid black; background-color: whitesmoke;" onclick={operator_update(1)}>{ "+" }</button>
        </div>
        </div>
        </p>
        </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<Calculator>::new().render();
}