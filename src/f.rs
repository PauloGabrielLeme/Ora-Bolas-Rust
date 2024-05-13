use std::fs::File;
use std::io::{BufRead,BufReader};
use plotters::prelude::*;
use plotters::style::RGBColor;
use std::error::Error;


fn ler_arquivo(lista_t: &mut Vec<f64>,lista_x: &mut Vec<f64>,lista_y: &mut Vec<f64>)-> Result<(),std::io::Error>{
    let file = File::open("dados.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines().skip(1) {
        let line = line?;
        let columns: Vec<_> = line.trim().split_whitespace().collect();

        let t_value = columns[0].replace(',', ".").parse::<f64>().unwrap_or(0.0);
        let x_value = columns[1].replace(',', ".").parse::<f64>().unwrap_or(0.0);
        let y_value = columns[2].replace(',', ".").parse::<f64>().unwrap_or(0.0);

        lista_t.push(t_value);
        lista_x.push(x_value);
        lista_y.push(y_value);
    }
    
    Ok(())
}

fn posicao_mais_proxima(robo_y: &f64, lista_y: &Vec<f64>)->usize {
    let mut index = 0;
    let mut min_distance = f64::MAX;

    for (i, &y) in lista_y.iter().enumerate() {
        if y < *robo_y {
            let distance = (*robo_y - y).abs();
            if distance < min_distance {
                min_distance = distance;
                index = i;
            }
        }
    }

    index
}


fn calcular_distancia(x1:f64,y1:f64,x2:f64,y2:f64)->f64{
    let dx:f64 = x2-x1; 
    let dy:f64 = y2-y1;

    let r:f64 = (dx.powi(2) + dy.powi(2)).sqrt();
    r

}

fn calcular_tempo(distancia:f64,velocidade:f64)->f64{

    let r:f64 = distancia / velocidade;
    r

}

fn calcular_forca(massa:f64,aceleracao:f64)->f64{

    let r:f64 = massa * aceleracao;
    r

}

fn calcular_forca_com_atrito(forca:f64,atrito:f64)->f64{

    forca * atrito

}

fn calcular_posicao_x(t:f64)->f64{
    0.005*t.powi(3) + 1E-13*t.powi(3) + 0.5*t + 1.0
}

fn calcular_posicao_y(t:f64)->f64{
    -0.02*t.powi(3) + 0.9*t + 0.5
}

fn calcular_velocidade_x(t:f64)->f64{
    0.015*t.powi(3) - 0.0003*t + 0.5
}

fn calcular_velocidade_y(t:f64)->f64{
    -0.04*t + 0.9004
}

fn calcular_aceleracao_x(t:f64)->f64{
    0.03*t - 0.0006
}

fn calcular_aceleracao_y(t:f64)->f64{
    -0.04
}

fn trajetoria(robo_x: f64, robo_y: f64, lista_x: &[f64], lista_y: &[f64]) {
    // Criar área de desenho para o gráfico
    let root =
    BitMapBackend::new("trajetorias_bola_e_robo.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Criar gráfico cartesiano 2D com rótulos e plano de fundo
    let mut chart = ChartBuilder::on(&root)
        .caption("Trajetórias da Bola e do Robô", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..55.0, 0.0..15.0)
        .unwrap();

    // Plotar trajetória da bola com legenda
    chart
        .draw_series(LineSeries::new(
            lista_x.iter().zip(lista_y.iter()).map(|(&x, &y)| (x, y)),
            &RGBColor(255, 0, 0), // Vermelho para a trajetória da bola
        ))
        .unwrap()
        .label("Trajetória da Bola")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255, 0, 0)));

    // Plotar trajetória do robô com legenda
    chart
        .draw_series(LineSeries::new(
            vec![(0.0, 0.0), (robo_x, robo_y)],
            &RGBColor(0, 0, 255), // Azul para a trajetória do robô
        ))
        .unwrap()
        .label("Trajetória do Robô")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0, 0, 255)));

    // Mostrar a legenda no gráfico
    chart.configure_series_labels().draw().unwrap();

    // Configurar automaticamente os ticks do eixo X
    chart
        .configure_mesh()
        .x_desc("Posição X (cm)")
        .y_desc("Posição Y (cm)")
        .draw()
        .unwrap();

    // Salvar o gráfico como imagem
    root.present().unwrap();

}


fn posicoes(lista_x: &[f64],lista_y: &Vec<f64>,lista_t: &Vec<f64>,robo_x:f64,robo_y:f64,tempo_robo_cheguei:f64){
    // Criar área de desenho para o gráfico
    let root = BitMapBackend::new("posicoes_bola_e_robo.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Criar gráfico cartesiano 2D com rótulos e plano de fundo
    let mut chart = ChartBuilder::on(&root)
        .caption("Posições da Bola e do Robô", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..25.0, 0.0..55.0)
        .unwrap(); 

    // Plotar trajetória da bola (Posição X)
    chart
        .draw_series(LineSeries::new(
            lista_t.iter().zip(lista_x.iter()).map(|(&t, &x)| (t, x)),
            &RGBColor(255, 0, 0),
        ))
        .unwrap()
        .label("Posição X da Bola")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255, 0, 0)));

    // Plotar trajetória do robô (Posição Y)
    chart
        .draw_series(LineSeries::new(
            lista_t.iter().zip(lista_y.iter()).map(|(&t, &y)| (t, y)),
            &RGBColor(0, 0, 255),
        ))
        .unwrap()
        .label("Posição Y da Bola")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0, 0, 255)));

    // Adicionar ponto representando a posição X do robô
    chart
        .draw_series(std::iter::once(Circle::new(
            (tempo_robo_cheguei, robo_x),
            5,
            Into::<ShapeStyle>::into(&RED),
        )))
        .unwrap()
        .label("Posição X do Robô")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255, 0, 0)));

    // Adicionar ponto representando a posição Y do robô
    chart
        .draw_series(std::iter::once(Circle::new(
            (tempo_robo_cheguei, robo_y),
            5,
            Into::<ShapeStyle>::into(&BLUE),
        )))
        .unwrap()
        .label("Posição Y do Robô")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0, 0, 255)));

    // Configurar rótulos das séries e desenhar legenda
    chart.configure_series_labels().draw().unwrap();

    // Configurar e desenhar malha do gráfico
    chart
        .configure_mesh()
        .x_desc("Posição X (cm)")
        .y_desc("Posição Y (cm)")
        .draw()
        .unwrap();
}
    



fn velocidade(lista_x: &[f64],lista_y: &Vec<f64>,lista_t: &Vec<f64>){
    // Criar área de desenho para o gráfico
    let root = BitMapBackend::new("velocidade_bola.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();


    let mut chart = ChartBuilder::on(&root)
        .caption("Velocidade da Bola em Relação Ao Tempo", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..25.0, 0.0..10.0)
        .unwrap(); 

    chart
        .draw_series(LineSeries::new(
            lista_t.iter().zip(lista_x.iter()).map(|(&x, &y)| (x, y)),
            &RGBColor(255, 0, 0), 
        ))
        .unwrap()
        .label("Velocidade No Eixo X da Bola")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255,0,0)));

    chart
        .draw_series(LineSeries::new(
            lista_t.iter().zip(lista_y.iter()).map(|(&x, &y)| (x, y)),
            &RGBColor(0, 0, 255),
        ))
        .unwrap()
        .label("Velocidade No Eixo Y da Bola")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0,0,255)));


    chart.configure_series_labels().draw().unwrap();

    chart
        .configure_mesh()
        .x_desc("Tempo (s)")
        .y_desc("Velocidade (m/s)")
        .draw()
        .unwrap();

}

fn aceleracao(lista_x: &[f64],lista_y: &Vec<f64>,lista_t: &Vec<f64>){
    // Criar área de desenho para o gráfico
    let root = BitMapBackend::new("aceleração_bola.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Criar gráfico cartesiano 2D com rótulos e plano de fundo
    let mut chart = ChartBuilder::on(&root)
        .caption("Aceleração da Bola em Relação Ao Tempo", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..25.0, -0.5..1.0)
        .unwrap(); // Ajuste o intervalo do eixo X para incluir os valores desejados

    chart
        .draw_series(LineSeries::new(
            lista_t.iter().zip(lista_x.iter()).map(|(&x, &y)| (x, y)),
            &RGBColor(255, 0, 0), // Vermelho para a trajetória da bola
        ))
        .unwrap()
        .label("Aceleração No Eixo X da Bola")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255,0,0)));

    chart
        .draw_series(LineSeries::new(
            lista_t.iter().zip(lista_y.iter()).map(|(&x, &y)| (x, y)),
            &RGBColor(0, 0, 255), // Vermelho para a trajetória da bola
        ))
        .unwrap()
        .label("Acelaração No Eixo Y da Bola")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0,0,255)));


    chart.configure_series_labels().draw().unwrap();

    chart
        .configure_mesh()
        .x_desc("Tempo (s)")
        .y_desc("Aceleração (m/s²)")
        .draw()
        .unwrap();

}

fn distancia_rel(distanci_ra:&[f64],tempo_velocidade:&[f64]){
    // Criar área de desenho para o gráfico
    let root = BitMapBackend::new("distancia_relativa.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Criar gráfico cartesiano 2D com rótulos e plano de fundo
    let mut chart = ChartBuilder::on(&root)
        .caption("Distância Relativa", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..25.0, 0.0..50.0)
        .unwrap(); // Ajuste o intervalo do eixo X para incluir os valores desejados

    chart
        .draw_series(LineSeries::new(
            tempo_velocidade.iter().zip(distanci_ra.iter()).map(|(&x, &y)| (x, y)),
            &RGBColor(255, 0, 0), // Vermelho para a trajetória da bola
        ))
        .unwrap()
        .label("Distância Relativa")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255,0,0)));


    chart.configure_series_labels().draw().unwrap();

    chart
        .configure_mesh()
        .x_desc("Tempo (s)")
        .y_desc("Distancia Relatica (cm)")
        .draw()
        .unwrap();
}

fn posicao_bola_relacao_tempo(posicoes_x:&[f64],posicoes_y:&[f64],tempo_grafico:&[f64]){
    // Criar área de desenho para o gráfico
    let root = BitMapBackend::new("posicao_bola_relacao_tempo.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Criar gráfico cartesiano 2D com rótulos e plano de fundo
    let mut chart = ChartBuilder::on(&root)
        .caption("Posição da Bola Em Relação Ao Tempo", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..1.75, 0.0..2.0)
        .unwrap(); // Ajuste o intervalo do eixo X para incluir os valores desejados

    chart
        .draw_series(LineSeries::new(
            tempo_grafico.iter().zip(posicoes_x.iter()).map(|(&x, &y)| (x, y)),
            &RGBColor(255, 0, 0), // Vermelho para a trajetória da bola
        ))
        .unwrap()
        .label("Posicao X Da Bola (Calculada)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255,0,0)));
    
    chart
        .draw_series(LineSeries::new(
            tempo_grafico.iter().zip(posicoes_y.iter()).map(|(&x, &y)| (x, y)),
            &RGBColor(0, 0, 255), // Vermelho para a trajetória da bola
        ))
        .unwrap()
        .label("Posicao Y Da Bola (Calculada)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0,0,255)));


    chart.configure_series_labels().draw().unwrap();

    chart
        .configure_mesh()
        .x_desc("Tempo (s)")
        .y_desc("Posição (cm)")
        .draw()
        .unwrap();


}

pub fn calculo_total(robo_x:&mut f64,robo_y:&mut f64){
    let mut lista_t = Vec::new();
    let mut lista_x = Vec::new();
    let mut lista_y = Vec::new();

    if let Err(err) = ler_arquivo(&mut lista_t, &mut lista_x, &mut lista_y) {
        eprintln!("Erro ao ler o arquivo: {}", err);
        return;
    }

    let i: usize;
    i = posicao_mais_proxima(robo_y,&mut lista_y);
    let mut posicao_y:f64 = lista_y[i];
    posicao_y = posicao_y / 2.0;

    println!("Posicao Mais Proxima Da Bola Ao Robo = {:.4} cm",posicao_y);

    let robo_velocidade:f64 = 2.8; //m/s
    let robo_aceleracao:f64 = 2.8; //m/s²
    let peso:f64 = 4.6; //N
    let massa:f64 = 0.46; //g

    let distancia_robo_bola = calcular_distancia(1.0, posicao_y, 1.0, 0.5); 
    let tempo_robo_cheguei:f64 = calcular_tempo(distancia_robo_bola,robo_velocidade);
    let forca:f64 = calcular_forca(massa,robo_aceleracao);
    let forca_com_atrito1:f64=calcular_forca_com_atrito(peso,0.2);
    let forca_com_atrito2:f64=calcular_forca_com_atrito(peso,0.5);
    let forca_com_atrito3:f64=calcular_forca_com_atrito(peso,0.7);

    println!("Distância da origem da bola até a interceptação = {:.4} cm",distancia_robo_bola);
    let distancia_bola_inicial:f64 = calcular_distancia(1.01,0.508,1.0,0.5);
    println!("Distância inicial da bola = {:.4} cm",distancia_bola_inicial);
    let distancia_bola_final:f64 = calcular_distancia(9.0, 5.3, 1.0, 0.5);
    println!("Distância final da bola = {:.4} cm",distancia_bola_final);
    let distancia_inicial_robo:f64 = calcular_distancia(0.0,0.0,*robo_x,*robo_y);
    println!("Distância inicial do robô = {:.4} cm",distancia_inicial_robo);
    let velocidade_media_inicial_bola:f64 = distancia_bola_inicial / 0.2;
    println!("Velocidade Media Incial da Bola = {:.4} m/s",velocidade_media_inicial_bola);
    let velocidade_media_final_bola:f64 = (distancia_bola_final/0.2)/2.0;
    println!("Velocidade Media Final da Bola = {:.4} m/s",velocidade_media_final_bola);
    let aceleracao_media_inicial_bola:f64 = velocidade_media_inicial_bola / 0.2;
    println!("Aceleracao Media Incial da Bola = {:.4} m/s²",aceleracao_media_inicial_bola);
    let aceleracao_media_final_bola:f64 = velocidade_media_final_bola / 0.2;
    println!("Aceleracao Media Final da Bola = {:.4} m/s²",aceleracao_media_final_bola);
    println!("Forca = {:.4} N",forca);
    println!("FaT Considerando Coeficiente de Atrito Estatico de 0.2 N = {:.4} N",forca_com_atrito1);
    println!("FaT Considerando Coeficiente de Atrito Estatico de 0.5 N = {:.4} N",forca_com_atrito2);
    println!("FaT Considerando Coeficiente de Atrito Estatico de 0.7 N = {:.4} N",forca_com_atrito3);
    

    trajetoria(*robo_x,*robo_y,&lista_x,&lista_y);
    posicoes(&lista_x,&lista_y,&lista_t,*robo_x,*robo_y,tempo_robo_cheguei);

    let mut velocidade_bola_x = Vec::with_capacity(lista_t.len() - 1);

    for i in 0..(lista_t.len() - 1) {
        let velocidade = (lista_x[i + 1] - lista_x[i]) / (lista_t[i + 1] - lista_t[i]);
        velocidade_bola_x.push(velocidade);
    }

    let mut velocidade_bola_y = Vec::with_capacity(lista_t.len() - 1);

    for i in 0..(lista_t.len() - 1) {
        let velocidade = (lista_y[i + 1] - lista_y[i]) / (lista_t[i + 1] - lista_t[i]);
        velocidade_bola_y.push(velocidade);
    }

    let mut tempo_velocidade = Vec::with_capacity(lista_t.len() - 1);

    for i in 0..(lista_t.len() - 1) {
        let tempo_medio = (lista_t[i + 1] + lista_t[i]) / 2.0;
        tempo_velocidade.push(tempo_medio);
    }

    velocidade(&velocidade_bola_x,&velocidade_bola_y,&tempo_velocidade);

    let mut aceleracao_bola_x = Vec::with_capacity(lista_t.len() - 2);

    for i in 0..(lista_t.len() - 2) {
        let aceleracao = (velocidade_bola_x[i + 1] - velocidade_bola_x[i]) / (lista_t[i + 1] - lista_t[i]);
        aceleracao_bola_x.push(aceleracao);
    }

    let mut aceleracao_bola_y = Vec::with_capacity(lista_t.len() - 2);

    for i in 0..(lista_t.len() - 2) {
        let aceleracao = (velocidade_bola_y[i + 1] - velocidade_bola_y[i]) / (lista_t[i + 1] - lista_t[i]);
        aceleracao_bola_y.push(aceleracao);
    }

    let mut tempo_aceleracao = Vec::with_capacity(lista_t.len() - 2);

    for i in 0..(lista_t.len() - 2) {
        let tempo_medio = (lista_t[i + 1] + lista_t[i]) / 2.0;
        tempo_aceleracao.push(tempo_medio);
    }

    aceleracao(&aceleracao_bola_x,&aceleracao_bola_y,&tempo_aceleracao);

    let mut distancia_relativa = Vec::with_capacity(lista_x.len() - 1);

    for i in 0..(lista_x.len() - 1) {
        let distancia = calcular_distancia(*robo_x, *robo_y, lista_x[i], lista_y[i]);
        distancia_relativa.push(distancia);
    }

    distancia_rel(&distancia_relativa,&tempo_velocidade);

    let mut tempo_grafico = Vec::new();
    let limite = (tempo_robo_cheguei * 100.0) as usize;

    for i in 0..limite {
        let tempo = i as f64 / 100.0;
        tempo_grafico.push(tempo);
    }

    // Utilizando um loop 'for' para calcular as posições x para cada tempo em 'tempo_grafico'
    let mut posicoes_x = Vec::new();

    // Iterar sobre cada tempo em 'tempo_grafico'
    for &tempo in &tempo_grafico {
        // Calcular a posição x para o tempo atual
        let posicao_x = calcular_posicao_x(tempo);
        // Adicionar a posição x calculada ao vetor 'posicoes_x'
        posicoes_x.push(posicao_x);
    }    

    // Utilizando um loop 'for' para calcular as posições x para cada tempo em 'tempo_grafico'
    let mut posicoes_y = Vec::new();

    // Iterar sobre cada tempo em 'tempo_grafico'
    for &tempo in &tempo_grafico {
        // Calcular a posição x para o tempo atual
        let posicao_y = calcular_posicao_y(tempo);
        // Adicionar a posição x calculada ao vetor 'posicoes_x'
        posicoes_y.push(posicao_y);
    }

    posicao_bola_relacao_tempo(&posicoes_x, &posicoes_y, &tempo_grafico);

}
