/*
TODOS: arrumar bugs deixar zerado
 - quando o slot esta usado não pode plantar  OK
 - não spawnar coisas em slots usados OK

 - quando pegar a carta ela vem pra frente, poder reorganizar elas assim ok
    - Funciona, qdo vc solta o botao o z da carta aumenta

 fazer uma animação qdo colhe a planta dela ir até o mercado e desaparecer num efeito de partículas  ok
    - feito


fazer uma animação usando tween que faz as coisas crescerem 
e diminuirem de tamanho uma vez como se tivessem pulsando, e
 quando apertar as coisas fazem isso



 fazer sistema de pedidos <-----

 fazer sistema de resultados

 fazer mercado
  - quando  chamar monta a tela com as cartas escolhidas rnd para vender

 fazer transmutador
    

 introduzir as plantas novas atraves do mercado


 QUANDO TUDO ISSO TIVER PRONTO - ITCH 
*/


#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(unreachable_patterns)]
#![allow(for_loops_over_fallibles)]


use bevy::math::vec3;
use bevy::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, *};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::query::QueryData;
use bevy::window::PresentMode;
use bevy::window::WindowPlugin;
use rand::*;

const LAYER_0: f32 = 0.0;//FUNDO
const LAYER_1: f32 = 0.1;//UI
const LAYER_2: f32 = 0.2;//plantas, ninhos, TEXTO UI
const LAYER_2_1: f32 = 0.21; // MARCADORES
const LAYER_3: f32 = 0.3;//Insetos
const LAYER_3_0: f32 = 0.32;//marcadores insetos
const LAYER_3_1: f32 = 0.31;//cartas
const LAYER_4: f32 = 0.4;//Telas e particulas e marcadores
const LAYER_5: f32 = 0.5;//Overlays
const LAYER_6: f32 = 0.6;// 



const ABELHA_VEL_X: f32 = 400.;
const ABELHA_VEL_Y: f32 = 400.;

const ABELHA_VEL_X2: f32 = 400.;
const ABELHA_VEL_Y2: f32 = 400.;

const ABELHA_ROT_MULT: f32 = 5.1;
const ABELHA_ROT_OFFSET: f32 = 1500.5;

const ABELHA_MULT_X: f32 = 5.0;
const ABELHA_MULT_Y: f32 = 0.5;

const MOSCA_VEL_X: f32 = 400.;
const MOSCA_VEL_Y: f32 = 400.;

const MOSCA_MULT_X: f32 = 5.0;
const MOSCA_MULT_Y: f32 = 0.5;


const DIALOGO_VEL_X: f32 = 1500.;
const DIALOGO_VEL_Y: f32 = 1600.;





const ANIMA_OFFSET1: f32 = 1800.;
const ANIMA_OFFSET2: f32 = 1900.;

const ANIMA_OFFSET3: f32 = 1300.;
const ANIMA_OFFSET4: f32 = 1500.;

const ANIMA_MULT_TIMER1: f32 = 5.0;
const ANIMA_MULT_TIMER2: f32 = 5.0;

const TEMPO_DIA: f32 = 24.0;
const TEMPO_RELOGIO: f32 = TEMPO_DIA / 4.0; 

const TEMPO_SECA_PLANTA: f32 = TEMPO_DIA / 2.0;
const TEMPO_CRESCE_PLANTA: f32 = TEMPO_DIA / 3.0;


const GOLD_INICIAL: u32 = 20;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum AppState {
    #[default]
    Startup,
    InGame,
    Paused,
    GameOver,
}


#[derive(Resource)]
struct TimerCrescePlanta(Timer);

#[derive(Resource)]
struct TimerSecaPlanta(Timer);

#[derive(Resource)]
struct TimerRelogio(Timer);

#[derive(Resource )]
struct Gold(u32);

#[derive(Resource)]
struct Inventario {
    itens: Vec<ItemInventario>,
}

#[derive(Resource)]
struct ItemInventario{
    tipo: TipoPlantas,
    quantidade: u32,
    visivel: bool,
}




//#[derive(Component )]
//struct SlotAgua <'a> (&'a mut Handle<Image>);
//#[derive(Component, Clone, PartialEq)]
//struct SlotAgua <'a> (&'a Entity);




//-------------------------------- ENUMS




#[derive(Component, Clone, Copy)]
enum TiposCarta {
    Semente,
    Agua,
    Guano,
}   

#[derive(Component, Clone, PartialEq)]
enum EstadoPlanta { 
    Semente,
    Pequena,
    Media,
    Adulta,
    Pronta,
}

#[derive(Component, Clone, Copy, PartialEq)]
enum FaseDia{
    MeioManha,
    MeioDia,
    MeioTarde,
    InicioNoite,
}


#[derive(Component, Clone, Copy, PartialEq, Debug)]
enum TipoPlantas{
    CouveRosa,
    Hedrazeba,
    GargomiloMiudo,
    FolhaGorda,
    CenouraPimenta,
    FlorAmarela,
}




#[derive(Component, Clone, PartialEq)]
enum EstadoRegaPlanta {
    BemMolhada,
    Molhada,
    Normal,
    QuaseSeca,
    Seca,
    Stressada,
}

#[derive(Component, Clone, PartialEq)]
enum EstadoAbelha {
    IndoFlor,
    ProcurandoFlor,
    IndoNinho,
    TirandoNectar,
    GuardandoNectar,
}

#[derive(Component, Clone)]
enum TipoInseto {
    Mosca,
    Lagarta,
    Pulgao,
    Besouro,
    Joaninha,
    Formiga,
    Aranha,
    Cigarra,
    Gafanhoto,
    Percevejo,
    Broca,
    Tripes,
    Cochonilha,
    Vaquinha,
    Mosquito,
}


//-------------------------------- COMPONENTES

#[derive(Component, Clone)]
struct RefTextoPedido(Entity);


#[derive(Component, Clone)]
struct ItemInventarioUI(TipoPlantas);

#[derive(Component, Clone)]
struct TextoItemInventarioUI(TipoPlantas);


#[derive(Component, Clone)]
struct Dialogo;

#[derive(Component, Clone)]
struct TelaGameOver;

#[derive(Component, Clone )]
struct MarcadorAgua(u32);

#[derive(Component, Clone, Debug)]
struct ItemPedido{
    tipo: TipoPlantas,
    quantidade: u32,
    valor: u32,
}

#[derive(Component, Clone, QueryData)]
struct Mel;

#[derive(Component, Clone, QueryData)]
struct MateriaPrima(Entity);

#[derive(Component, Clone, QueryData)]
struct RefSlot(Entity); //slot que a planta esta plantada


#[derive(Component, Clone, QueryData)]
struct RefMateriaPrima(Entity);

#[derive(Component, Clone)]
struct FlagApaga;

#[derive(Component, Clone)]
struct FlorAmarela;

#[derive(Component, Clone)]
struct Target(Entity);

#[derive(Component, Clone)]
struct SlotGuano (Entity);

#[derive(Component, Clone)]
struct Planta(EstadoPlanta, EstadoRegaPlanta);

#[derive(Component, Clone)]
struct RitmodeCrescimento (u32);

#[derive(Component, Clone)]
struct Dragging;

#[derive(Component, Clone)]
struct Semeado;


#[derive(Component, Clone)]
struct Mercado;

#[derive(Component, Clone)]
struct Slot;


#[derive(Component, Clone)]
struct MaturacaoAtual(u32);


#[derive(Component, Clone)]
struct SlotAgua (Entity);


#[derive(Component, Clone)]
struct OverlayCor;


#[derive(Component, Clone)]
struct Selecionado (bool);

#[derive(Component, Clone)]
struct Usada;

#[derive(Component, Clone)]
struct Usada2;

#[derive(Component, Clone)]
struct Usada3;

#[derive(Component, Clone)]
struct TipoCarta(TiposCarta);


#[derive(Component, Clone, QueryFilter)]
struct PlantaMorta;

#[derive(Component, Clone, QueryFilter)]
struct EstressadaFlag;


#[derive(Component, Clone)]
struct Pedidos;

#[derive(Component, Clone, Copy)]
struct EstruturaMaturacao
{
    semente: u32,
    pequena: u32,
    media: u32,
    adulta: u32,
    pronta: u32,
}

#[derive(Component, Clone, Copy)]
struct EstruturaConsumoAgua
{
    semente: u32,
    pequena: u32,
    media: u32,
    adulta: u32,
    pronta: u32,
}




#[derive(Component, Clone)]
struct TipoPlanta(TipoPlantas);

#[derive(Component, Clone, Copy)]
struct Relogio (FaseDia);

#[derive(Component, Clone)]
struct Dia (u32);

#[derive(Component, Clone)]
struct UIDinheiro (u32);

#[derive(Component, Clone)]
struct UIMercado;



#[derive(Component, Clone)]
struct Resultados;

#[derive(Component, Clone)]
struct Cosmetico;

#[derive(Component, Clone)]
struct CosmeticoFlag;

#[derive(Component, Clone)]
struct EstadoAbelhaAtual(EstadoAbelha);

#[derive(Component, Clone)]
struct SlotGuanoFlag;

#[derive(Component, Clone)]
struct Inseto(TipoInseto);

#[derive(Component, Clone)]
struct Grito;

#[derive(Component, Clone)]
struct Ninho;

#[derive(Component, Clone)]
struct Abelha;

#[derive(Component, Clone)]
struct Mensagem;

//-------------------------------- EVENTOS

#[derive(Event)]
struct eAdicionaUmaPlantaInventario(TipoPlantas);

#[derive(Event)]
struct eSubtraiUmaPlantaInventario(TipoPlantas);


#[derive(Event)]
struct eAbreDialogo();

#[derive(Event)]
struct eSpawnaCosmetico();

#[derive(Event)]
struct eAbrePedidos();

#[derive(Event)]
struct eAbreMensagem();

#[derive(Event)]
struct eGrita();

#[derive(Event)]
struct eAbreTransmutador();

#[derive(Event)]
struct eAbreResultados();

#[derive(Event)]
struct ColheuPlanta(TipoPlantas);//entidade da planta e transform dela para fazer o spawn da planta colhida (em forma de carta)   

#[derive(Event)]
struct eSpawnaCarta();

#[derive(Event)]
struct eSpawnaGuano();

#[derive(Event)]
struct eSpawnaSemente(TipoPlantas);

#[derive(Event)]
struct eVender();




fn main() {
    let mut app = App::new();
    
    app.insert_resource(AssetMetaCheck::Never)
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))

    .add_plugins(TweeningPlugin)
    .add_plugins(DefaultPickingPlugins)
    .init_state::<AppState>()
    .add_event::<eSubtraiUmaPlantaInventario>()
    .add_event::<eAdicionaUmaPlantaInventario>()
    .add_event::<eAbreDialogo>()
    .add_event::<eVender>()
    .add_event::<Explosion>()
    .add_event::<eGrita>()
    .add_event::<eSpawnaCosmetico>()
    .add_event::<eAbrePedidos>()
    .add_event::<eAbreResultados>()
    .add_event::<eAbreMensagem>()
    .add_event::<eSpawnaCarta>()
    .add_event::<eSpawnaSemente>()
    .add_event::<eAbreTransmutador>()
    .add_event::<eSpawnaGuano>()
    .add_event::<ColheuPlanta>()

    //----------------------- Recursos
    
    .insert_resource(Time::<Fixed>::from_seconds(0.25))
    .insert_resource(Gold(GOLD_INICIAL))
    .insert_resource(Inventario{ itens: Vec::new()})
    
    .insert_resource(TimerRelogio(
        Timer::from_seconds(TEMPO_RELOGIO, TimerMode::Repeating)))
    .insert_resource(TimerCrescePlanta(
                        Timer::from_seconds(TEMPO_CRESCE_PLANTA, TimerMode::Repeating)))
    .insert_resource(TimerSecaPlanta(
        Timer::from_seconds(TEMPO_SECA_PLANTA, TimerMode::Repeating)))

     //----------------------- Sistemas
    
    .add_systems(Startup, Setup)

    //chama mensagem ja nao serve pra nada
    .add_systems(PostStartup,(ChamaMensagem, TelaInicio ).chain()) //arrumar
    
    .add_systems(Update, particle_system)
    .add_systems(Update, move_particles)
    
    //.add_systems(Update, TelaInicio)
    .add_systems(Update, AdicionaRemoveUmaPlanta)
    .add_systems(Update, AbreDialogo)
    .add_systems(Update, AtualizaInventario)
    .add_systems(Update, TelaFim.run_if(in_state(AppState::GameOver)))
    .add_systems(Update, Grita.run_if(in_state(AppState::InGame)))
    .add_systems(Update, MercadoDetectaCartas.run_if(in_state(AppState::InGame)))
    .add_systems(Update, AtualizaUiGold.run_if(in_state(AppState::InGame)))
   // .add_systems(Update, Vender.run_if(in_state(AppState::InGame))) 
    .add_systems(Update, SlotDetectaCartas.run_if(in_state(AppState::InGame)))
    .add_systems(Update, AtualizaRelogio.run_if(in_state(AppState::InGame)))
    .add_systems(Update, AbreResultados.run_if(in_state(AppState::InGame)))
    .add_systems(Update, AbreMensagem)
    .add_systems(Update, AbrePedidos.run_if(in_state(AppState::InGame)))
    //.add_systems(Update, AbreTransmutador.run_if(in_state(AppState::InGame)))
    .add_systems(Update, Limpa.run_if(in_state(AppState::InGame)))
    .add_systems(Update, Colhe.run_if(in_state(AppState::InGame)))
    .add_systems(Update, MovimentaMosca.run_if(in_state(AppState::InGame)))
    .add_systems(Update, MovimentaAbelha.run_if(in_state(AppState::InGame)))
    //.add_systems(Update, DetectaSimbiose)
    .add_systems(Update, SpawnaCosmetico.run_if(in_state(AppState::InGame)))
    .add_systems(Update, CrescePlanta.run_if(in_state(AppState::InGame)))
    .add_systems(Update, DespawnaMercado)
    .add_systems(Update, MateriaPrimaSegueAbelha)
    .add_systems(Update, SpawnaCarta.run_if(in_state(AppState::InGame))) // por que tem um espawna carta e tem esse tb
    .add_systems(Update, SpawnaGuano.run_if(in_state(AppState::InGame)))
    .add_systems(Update, SecaPlanta.run_if(in_state(AppState::InGame)))
    .add_systems(Update, AnimaNinho)
    .add_systems(Update, Pause)
    .add_systems(Update, AnimaPlantas);
    app.run();
}

fn TelaInicio (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,
    
)

{

    let tween1 = Tween::new(
        EaseFunction::QuadraticInOut,
        std::time::Duration::from_millis(500),
        TransformPositionLens{
            start:Vec3::new(0., 500., LAYER_6),
            end:Vec3::new(0., 0., LAYER_6),
        }
    );

    
    commands.spawn((

        Resultados, //criar um tipo "Janela" ou tela, e usar um só sistema pra fechar todas
        Animator::new(tween1),
        SpriteBundle {
            texture: asset_server.load("./Hectares/Cards/Misc/Abertura.png"),
            sprite: Sprite{
            
                custom_size: Some(Vec2::new(500.0, 903.0)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., LAYER_6),
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Click>>::run(ComecaJogo),
    ));




}


fn TelaFim (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,

    query: Query<(Entity, &TelaGameOver)>,
    
)

{


 for (entidade, _) in query.iter() {
        return;
    }
    
    commands.spawn((

        TelaGameOver, //criar um tipo "Janela" ou tela, e usar um só sistema pra fechar todas
        
        SpriteBundle {
            texture: asset_server.load("Hectares/Cards/Misc/fim.png"),
            sprite: Sprite{
            
                custom_size: Some(Vec2::new(500.0, 903.0)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., LAYER_6),
            ..default()
        },
        PickableBundle::default(),
       // On::<Pointer<Click>>::run(ComecaJogo),
    ));



}



//MUDAR O NOME DA TELA PARA CORRETA, ESTA COMO RESULTADOS
fn ComecaJogo (
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,
    query: Query<(Entity, &Resultados)>,
    mut eventosMensagem: EventWriter<eAbreMensagem>,
    mut eventosSemente: EventWriter<eSpawnaSemente>,
    mut eventosCos: EventWriter<eSpawnaCosmetico>,
    mut inventario: ResMut<Inventario>,
)

{
    inventario.itens.push(ItemInventario{tipo: TipoPlantas::CouveRosa, quantidade: 2, visivel: false}); 
    inventario.itens.push(ItemInventario{tipo: TipoPlantas::Hedrazeba, quantidade: 1, visivel: false});
    inventario.itens.push(ItemInventario{tipo: TipoPlantas::GargomiloMiudo, quantidade: 3, visivel: false});
    inventario.itens.push(ItemInventario{tipo: TipoPlantas::FolhaGorda, quantidade: 4, visivel: false});
    for (entidade, _) in query.iter() {
        commands.entity(entidade).despawn();
    }
    app_state.set(AppState::InGame);
    eventosMensagem.send(eAbreMensagem());
   // eventosSemente.send(eSpawnaSemente(TipoPlantas::CouveRosa));
   // eventosSemente.send(eSpawnaSemente(TipoPlantas::Hedrazeba));
   // eventosSemente.send(eSpawnaSemente(TipoPlantas::GargomiloMiudo));
  //  eventosSemente.send(eSpawnaSemente(TipoPlantas::FolhaGorda));
  //  eventosSemente.send(eSpawnaSemente(TipoPlantas::FlorAmarela));
    for x in 0..9 {
    
        eventosCos.send(eSpawnaCosmetico());
    }


}

//arrumnar isso !!!!!!!!!!!!!!
fn ChamaMensagem(
    eventos: EventWriter<eAbreMensagem>,
    mut eventosCos: EventWriter<eSpawnaCosmetico>,
)
{
//    eventos.send(eAbreMensagem());
    for x in 0..9 {
    
        eventosCos.send(eSpawnaCosmetico());
    }

}

fn Pause (keys: Res<ButtonInput<KeyCode>>, 
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>)
{

    match app_state_atual.get() {
        AppState::Paused => {
            if keys.just_pressed(KeyCode::Space) {
                app_state.set(AppState::InGame);
            }
        },
        AppState::InGame => {
            if keys.just_pressed(KeyCode::Space) {
                app_state.set(AppState::Paused);
            }
        },
        _ => {}
    }


}


fn AnimaNinho(
    mut query: Query<(Entity, &mut Transform, &Ninho)>,
    time: Res<Time>,
)
{
    let time = time.elapsed_seconds() as f32;
    for (_,mut transform, _ ) in query.iter_mut() {
        transform.scale.x  += (time * ANIMA_MULT_TIMER1).sin() /ANIMA_OFFSET1 ;
        transform.scale.y  -= (time * ANIMA_MULT_TIMER2).sin() /ANIMA_OFFSET2 ;


    }
}



fn AnimaPlantas( 
    time: Res<Time>,

    mut query: Query<(Entity, &mut Transform, &mut Planta), (Without<CosmeticoFlag>, Without<PlantaMorta>)>,
    mut query2: Query<(Entity, &mut Transform), With<CosmeticoFlag>>,
   // mut qninho
    
)
{

    
    let time = time.elapsed_seconds() as f32;
    for (_,mut transform,_ ) in query.iter_mut() {
        transform.scale.y  += (time * ANIMA_MULT_TIMER1).sin() /ANIMA_OFFSET1 ;
        transform.scale.x  -= (time * ANIMA_MULT_TIMER2).sin() /ANIMA_OFFSET1 ;

    }
    for (_,mut transform ) in query2.iter_mut() {
        transform.scale.y  += (time * ANIMA_MULT_TIMER1).sin() /ANIMA_OFFSET3 ;
        transform.scale.x  -= (time * ANIMA_MULT_TIMER2).sin() /ANIMA_OFFSET4 ;

    }


}

fn UISlotAguaLvl(
    query: Query<(Entity, &SlotAgua)>,
    mut query2: Query<(Entity, &mut Handle<Image>, &SlotAgua)>,
    asset_server: ResMut<AssetServer>,
)
{
    for (entidade, slot) in query.iter() {
        for (entidade2, textura, slot2) in query2.iter_mut() {
            //if slot == slot2 {
              //  *textura = asset_server.load("Hectares/Cards/top-agua-cheia.png");
            //}
        }
    }
}






// se a planta fica seca estressada ela ganha uma flag, 
//na segunda flag ela morre e a imagem fica preta, ao colher uma planta que esta com esse status não vem nada nem dinheiro nem nada

fn SecaPlanta (
    mut timer: ResMut<TimerSecaPlanta>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut query: Query<(Entity, &mut Planta, &mut MarcadorAgua,   &TipoPlanta , &mut Transform, &mut Sprite,&mut Handle<Image>, &SlotAgua), (Without<PlantaMorta>,  Without<EstressadaFlag>)>,
    mut query2: Query<(Entity, &mut Handle<Image>), Without<SlotAgua>>,
    query3: Query<(Entity, &SlotGuanoFlag), Without<Usada>>,
    //mut QPlantaMorta: Query<(Entity, &PlantaMorta)>,
    mut qplantaestressada: Query<(Entity, &Planta,&mut Sprite, &mut EstressadaFlag)>,
  
)
{
    if timer.0.tick(time.delta()).just_finished() { //se o timer acabou atualiza as plantas
        for (entidade, planta,mut sprite, estressada) in qplantaestressada.iter_mut() {
            commands.entity(entidade).insert(PlantaMorta);
            sprite.color = Color::rgb(0.3, 0.2, 0.2);

        }



            for (entidade, mut planta,mut marcadoragua, tipoplanta ,  transform, sprite, handle, slotid ) in query.iter_mut() { //varre as plantas spawnadas no jogo

                
                match tipoplanta.0 {
                    TipoPlantas::CouveRosa => {
                     
                planta.1 = match planta.1 {   //muda o estado da rega da planta
                    EstadoRegaPlanta::BemMolhada => EstadoRegaPlanta::Molhada,
                    EstadoRegaPlanta::Molhada => EstadoRegaPlanta::Normal,
                    EstadoRegaPlanta::Normal => EstadoRegaPlanta::QuaseSeca,
                    EstadoRegaPlanta::QuaseSeca => EstadoRegaPlanta::Seca,
                    EstadoRegaPlanta::Seca => EstadoRegaPlanta::Stressada,
                    EstadoRegaPlanta::Stressada => EstadoRegaPlanta::Stressada,
                }; 
                
                    }
                    TipoPlantas::Hedrazeba => {
                        
                planta.1 = match planta.1 {   //muda o estado da rega da planta
                    EstadoRegaPlanta::BemMolhada => EstadoRegaPlanta::Molhada,
                    EstadoRegaPlanta::Molhada => EstadoRegaPlanta::Normal,
                    EstadoRegaPlanta::Normal => EstadoRegaPlanta::QuaseSeca,
                    EstadoRegaPlanta::QuaseSeca => EstadoRegaPlanta::Seca,
                    EstadoRegaPlanta::Seca => EstadoRegaPlanta::Stressada,
                    EstadoRegaPlanta::Stressada => EstadoRegaPlanta::Stressada,
                }; 
                
                
                    }
                    TipoPlantas::GargomiloMiudo => {
                  
                planta.1 = match planta.1 {   //muda o estado da rega da planta
                    EstadoRegaPlanta::BemMolhada => EstadoRegaPlanta::Molhada,
                    EstadoRegaPlanta::Molhada => EstadoRegaPlanta::Normal,
                    EstadoRegaPlanta::Normal => EstadoRegaPlanta::QuaseSeca,
                    EstadoRegaPlanta::QuaseSeca => EstadoRegaPlanta::Seca,
                    EstadoRegaPlanta::Seca => EstadoRegaPlanta::Stressada,
                    EstadoRegaPlanta::Stressada => EstadoRegaPlanta::Stressada,
                }; 
                
                    }
                    TipoPlantas::FolhaGorda => {
                
                        planta.1 = match planta.1 {   //muda o estado da rega da planta
                            EstadoRegaPlanta::BemMolhada => EstadoRegaPlanta::Molhada,
                            EstadoRegaPlanta::Molhada => EstadoRegaPlanta::Normal,
                            EstadoRegaPlanta::Normal => EstadoRegaPlanta::QuaseSeca,
                            EstadoRegaPlanta::QuaseSeca => EstadoRegaPlanta::Seca,
                            EstadoRegaPlanta::Seca => EstadoRegaPlanta::Stressada,
                            EstadoRegaPlanta::Stressada => EstadoRegaPlanta::Stressada,
                        }; 
                        
                    }
                    TipoPlantas::CenouraPimenta => {
                    
                planta.1 = match planta.1 {   //muda o estado da rega da planta
                    EstadoRegaPlanta::BemMolhada => EstadoRegaPlanta::Molhada,
                    EstadoRegaPlanta::Molhada => EstadoRegaPlanta::Normal,
                    EstadoRegaPlanta::Normal => EstadoRegaPlanta::QuaseSeca,
                    EstadoRegaPlanta::QuaseSeca => EstadoRegaPlanta::Seca,
                    EstadoRegaPlanta::Seca => EstadoRegaPlanta::Stressada,
                    EstadoRegaPlanta::Stressada => EstadoRegaPlanta::Stressada,
                }; 
                
                    }
                    TipoPlantas::FlorAmarela => {
                
                        planta.1 = match planta.1 {   //muda o estado da rega da planta
                            EstadoRegaPlanta::BemMolhada => EstadoRegaPlanta::Molhada,
                            EstadoRegaPlanta::Molhada => EstadoRegaPlanta::Normal,
                            EstadoRegaPlanta::Normal => EstadoRegaPlanta::QuaseSeca,
                            EstadoRegaPlanta::QuaseSeca => EstadoRegaPlanta::Seca,
                            EstadoRegaPlanta::Seca => EstadoRegaPlanta::Seca,
                            EstadoRegaPlanta::Stressada => EstadoRegaPlanta::Stressada,
                        }; 
                        
                    }

                }

                



                if planta.1 == EstadoRegaPlanta::Stressada {
                    commands.entity(entidade).insert(EstressadaFlag);
                }
                

                for (_, mut handle) in query2.get_mut(slotid.0) {
                    
                    match planta.1 { //atualiza a textura da planta
                        EstadoRegaPlanta::BemMolhada => {
                            *handle = asset_server.load("Hectares/Cards/Slot - Agua - Bem Molhada.png");
                        }
                        EstadoRegaPlanta::Molhada => {
                            *handle = asset_server.load("Hectares/Cards/Slot - Agua - Molhada.png");
                        }
                        EstadoRegaPlanta::Normal => {
                            *handle = asset_server.load("Hectares/Cards/Slot - Agua - Normal.png");
                        }
                        EstadoRegaPlanta::QuaseSeca => {
                            *handle = asset_server.load("Hectares/Cards/Slot - Agua - Quase Seca.png");
                        }
                        EstadoRegaPlanta::Seca => {
                            *handle = asset_server.load("Hectares/Cards/Slot - Agua - Seca.png");
                        }
                        EstadoRegaPlanta::Stressada => {
                            *handle = asset_server.load("Hectares/Cards/Slot - Agua - Estressada.png");
                        }
                    }
                    
                }

            

                for (entidade, _) in query3.iter(){
                    commands.entity(entidade).insert(Usada);
                }


                /*
                match planta.1 { //atualiza a textura da planta
                    EstadoRegaPlanta::Normal => {
                        sprite.color = Color::WHITE;
                        
                    }
                    EstadoRegaPlanta::BemMolhada => {
                        sprite.color = Color::rgb(0.5,  0.5, 1.0);
                    }
                    EstadoRegaPlanta::Molhada => {
                        sprite.color = Color::rgb(0.7, 0.7, 1.0);
                    }
                    EstadoRegaPlanta::QuaseSeca => {
                        sprite.color = Color::rgb(1.0, 0.7, 0.7);
                    }
                    EstadoRegaPlanta::Seca => {
                        sprite.color = Color::rgb(1.0, 0.4, 0.4);
                    }
                    EstadoRegaPlanta::Stressada => {
                        sprite.color = Color::rgb(0.3, 0.2, 0.2);
                    }
                }*/
        }
    }
}




fn AtualizaUiGold(
    ouro : ResMut<Gold>,

    mut query2: Query<(Entity, &mut Text, &UIDinheiro)>,
)
{

    if ouro.is_changed() {
        info!("Ouro mudou para {}", ouro.0);
        for (entidade2, mut text, _) in query2.iter_mut() {
            text.sections[0].value = ouro.0.to_string();
        }
    }
}


//plantas diferentes crescem em ritmos diferentes !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!


fn AtualizaRelogio(
    mut query: Query<(Entity, &mut Handle<Image>,  &mut Relogio)>,
    mut query2: Query<(Entity, &mut Sprite,  &OverlayCor)>,
    mut query3: Query<(Entity, &mut Text,  &mut Dia)>,
    asset_server: ResMut<AssetServer>,
    time: Res<Time>,
    eventos: EventWriter<eGrita>,
    mut eabrepedidos: EventWriter<eAbrePedidos>,
    mut eabreresultados: EventWriter<eAbreResultados>,


    mut commands: Commands,
    mut timer: ResMut<TimerRelogio>,
    mut eabredialogo: EventWriter<eAbreDialogo>,
   // world: &mut World,
   mut app_state: ResMut<NextState<AppState>>, 
   app_state_atual: Res<State<AppState>>,
    query4: Query<(Entity, &SlotGuanoFlag), With<Usada>>,
)
{

    

    if timer.0.tick(time.delta()).just_finished() { //se o timer acabou atualiza as plantas


      
        for (entidade, mut textura , mut relogio) in query.iter_mut() {

            //significa que passou um dia ...
            if relogio.0 == FaseDia::InicioNoite {
                for (_,mut texto,mut dia) in query3.iter_mut() {
                    dia.0 += 1;
                    texto.sections[0].value = dia.0.to_string();



                    if dia.0 == 3 {
                        //dia.0 = 0;
                        eabreresultados.send(eAbreResultados());
                    }
                    if dia.0 == 2 {
                        
                        app_state.set(AppState::GameOver);
                    }
                    
                    

            }
        }
            relogio.0 = match relogio.0 {
                FaseDia::MeioManha => {
                    FaseDia::MeioDia
                }
                FaseDia::MeioDia => {
                    FaseDia::MeioTarde                    
                }
                FaseDia::MeioTarde => {
                    FaseDia::InicioNoite
                }
                FaseDia::InicioNoite => {
                    FaseDia::MeioManha
                }
            };


            

        for (_,mut sprite, _) in query2.iter_mut() {
           match relogio.0 {
                FaseDia::MeioManha => {
                    //eabrepedidos.send(eAbrePedidos());  

                    sprite.color = Color::rgba(0.2, 0.4, 0.4, 0.8);
                }
                FaseDia::MeioDia => {
                    sprite.color = Color::rgba(0.9, 0.9, 0.4, 0.2);
                    //eventos.send(eGrita());
                    eabrepedidos.send(eAbrePedidos());  
                    
                }
                FaseDia::MeioTarde => {
                    //eabredialogo.send(eAbreDialogo());
                    sprite.color = Color::rgba(0.9, 0.5, 0.4, 0.5);
                }
                FaseDia::InicioNoite => {
                    sprite.color = Color::rgba(0.2, 0.3, 0.9, 0.8);
                    
                    
                    for (entidade, _) in query4.iter(){
                        commands.entity(entidade).despawn();
                    }
                    
                    
                
               }
             }
        }
        

            match relogio.0 {
                FaseDia::MeioManha => {
                    *textura = asset_server.load("Hectares/Cards/Meio da manha.png");
                }
                FaseDia::MeioDia => {
                    *textura = asset_server.load("Hectares/Cards/Meio dia.png");
                }
                FaseDia::MeioTarde => {
                    *textura = asset_server.load("Hectares/Cards/final da tarde.png");
                }
                FaseDia::InicioNoite => {
                    *textura = asset_server.load("Hectares/Cards/inicio da noite.png");
                }
            }
        }
    }
   
}



//faz a planta crescer e atualiza a textura da planta 

fn CrescePlanta (
    mut timer: ResMut<TimerCrescePlanta>,
    time: Res<Time>,
    asset_server: ResMut<AssetServer>,
    mut query: Query<(Entity, &mut Planta, &mut Transform, &mut Sprite, &mut Handle<Image>,&TipoPlanta, &EstruturaMaturacao, &mut MaturacaoAtual ), Without<PlantaMorta> >,
)
{
    if timer.0.tick(time.delta()).just_finished() { //se o timer acabou atualiza as plantas
            for (entidade, mut planta, transform, sprite, texture, _, estruturamaturacao, mut maturacaoatual) in query.iter_mut() { //varre as plantas spawnadas no jogo
                let mut podecrescer = false;
                if planta.0 != EstadoPlanta::Pronta {
                    maturacaoatual.0 += 1;
                    

                    if planta.0 == EstadoPlanta::Semente {
                        if maturacaoatual.0 >= estruturamaturacao.semente {
                           // maturacaoatual.0 = 0;
                            podecrescer = true;
                        }
                    }
                    if planta.0 == EstadoPlanta::Pequena {
                        if maturacaoatual.0 >= estruturamaturacao.pequena {
                            maturacaoatual.0 = 0;
                            podecrescer = true;
                        }
                    }
                    if planta.0 == EstadoPlanta::Media {
                        if maturacaoatual.0 >= estruturamaturacao.media {
                            maturacaoatual.0 = 0;
                            podecrescer = true;
                        }
                    }
                    if planta.0 == EstadoPlanta::Adulta {
                        if maturacaoatual.0 >= estruturamaturacao.adulta {
                            maturacaoatual.0 = 0;
                            podecrescer = true;
                        }
                    }
                    if planta.0 == EstadoPlanta::Pronta {
                        if maturacaoatual.0 >= estruturamaturacao.pronta {
                            maturacaoatual.0 = 0;
                            podecrescer = false;
                        }

                }
            }
                //coloca uma checagem antes e solta o evento de crescimento se passar a checagem


                    if podecrescer{

                        planta.0 = match planta.0 {   //muda o estado da planta
                        EstadoPlanta::Semente => EstadoPlanta::Pequena,
                        EstadoPlanta::Pequena => EstadoPlanta::Media,
                        EstadoPlanta::Media => EstadoPlanta::Adulta,
                        EstadoPlanta::Adulta => EstadoPlanta::Pronta,
                        EstadoPlanta::Pronta => EstadoPlanta::Pronta,
                    };       
            
                }
            }
    }
    for (entidade, planta, transform, sprite, mut texture, tiposplanta,_,_) in query.iter_mut() { //varre as plantas spawnadas no jogo       
        
        match planta.0 { //atualiza a textura da planta
            EstadoPlanta::Semente => {
                *texture = match tiposplanta.0 {
                    TipoPlantas::CouveRosa => asset_server.load("Hectares/Cards/Plantas/Couve Flor/1.png"),
                    TipoPlantas::Hedrazeba => asset_server.load("Hectares/Cards/Plantas/Hedrazeba/1.png"),
                    TipoPlantas::GargomiloMiudo => asset_server.load("Hectares/Cards/Plantas/Gargomilo Miudo/1.png"),
                    TipoPlantas::FolhaGorda => asset_server.load("Hectares/Cards/Plantas/Folha Gorda/1.png"),
                    TipoPlantas::CenouraPimenta => asset_server.load("Hectares/Cards/Plantas/Cenoura Pimenta/1.png"),
                    TipoPlantas::FlorAmarela => asset_server.load("Hectares/Cards/Plantas/Flor Amarela/1.png"),
                };
            }
            EstadoPlanta::Pequena => {
                *texture = match tiposplanta.0 {
                    TipoPlantas::CouveRosa => asset_server.load("Hectares/Cards/Plantas/Couve Flor/2.png"),
                    TipoPlantas::Hedrazeba => asset_server.load("Hectares/Cards/Plantas/Hedrazeba/2.png"),
                    TipoPlantas::GargomiloMiudo => asset_server.load("Hectares/Cards/Plantas/Gargomilo Miudo/2.png"),
                    TipoPlantas::FolhaGorda => asset_server.load("Hectares/Cards/Plantas/Folha Gorda/2.png"),
                    TipoPlantas::CenouraPimenta => asset_server.load("Hectares/Cards/Plantas/Cenoura Pimenta/2.png"),
                    TipoPlantas::FlorAmarela => asset_server.load("Hectares/Cards/Plantas/Flor Amarela/1.png"),
                };
            }
            EstadoPlanta::Media => {
                *texture = match tiposplanta.0 {
                    TipoPlantas::CouveRosa => asset_server.load("Hectares/Cards/Plantas/Couve Flor/3.png"),
                    TipoPlantas::Hedrazeba => asset_server.load("Hectares/Cards/Plantas/Hedrazeba/3.png"),
                    TipoPlantas::GargomiloMiudo => asset_server.load("Hectares/Cards/Plantas/Gargomilo Miudo/3.png"),
                    TipoPlantas::FolhaGorda => asset_server.load("Hectares/Cards/Plantas/Folha Gorda/3.png"),
                    TipoPlantas::CenouraPimenta => asset_server.load("Hectares/Cards/Plantas/Cenoura Pimenta/3.png"),
                    TipoPlantas::FlorAmarela => asset_server.load("Hectares/Cards/Plantas/Flor Amarela/1.png"),
                };
            }
            EstadoPlanta::Adulta => {
                *texture = match tiposplanta.0 {
                    TipoPlantas::CouveRosa => asset_server.load("Hectares/Cards/Plantas/Couve Flor/4.png"),
                    TipoPlantas::Hedrazeba => asset_server.load("Hectares/Cards/Plantas/Hedrazeba/4.png"),
                    TipoPlantas::GargomiloMiudo => asset_server.load("Hectares/Cards/Plantas/Gargomilo Miudo/4.png"),
                    TipoPlantas::FolhaGorda => asset_server.load("Hectares/Cards/Plantas/Folha Gorda/4.png"),
                    TipoPlantas::CenouraPimenta => asset_server.load("Hectares/Cards/Plantas/Cenoura Pimenta/4.png"),
                    TipoPlantas::FlorAmarela => asset_server.load("Hectares/Cards/Plantas/Flor Amarela/1.png"),
                };
            }
            EstadoPlanta::Pronta => {
                *texture = match tiposplanta.0 {
                    TipoPlantas::CouveRosa => asset_server.load("Hectares/Cards/Plantas/Couve Flor/5.png"),
                    TipoPlantas::Hedrazeba => asset_server.load("Hectares/Cards/Plantas/Hedrazeba/5.png"),
                    TipoPlantas::GargomiloMiudo => asset_server.load("Hectares/Cards/Plantas/Gargomilo Miudo/5.png"),
                    TipoPlantas::FolhaGorda => asset_server.load("Hectares/Cards/Plantas/Folha Gorda/5.png"),
                    TipoPlantas::CenouraPimenta => asset_server.load("Hectares/Cards/Plantas/Cenoura Pimenta/5.png"),
                    TipoPlantas::FlorAmarela => asset_server.load("Hectares/Cards/Plantas/Flor Amarela/1.png"),
                };
            }
        }
    }

    
  
}

//esse função detecta se existe uma carta sobre o mercado, se existir , e o jogador fizer um pointer<up> sobre ela, ela é despawnada
//faça duas queries, uma puxa onde esta o mercado, se a carta esta sobre ele, a outra puxa a carta, se o pointer<up> for sobre ela, ela é despawnada
fn MercadoDetectaCartas(
    mut commands: Commands,
    mut eventoExplosao: EventWriter<Explosion>,
    mut query: Query<(Entity, &Transform, &UIMercado)>,
    mut query2: Query<(Entity, &Transform, &Dragging, &TipoCarta)>,
    mut ouro: ResMut<Gold>,
)
{

    for (entidade, transform, _) in query.iter() {
        for (entidade2, transform2, _, _) in query2.iter() {
            let mut tempTransform = transform2.clone();
            tempTransform.translation.y += 50.0;
            tempTransform.translation.x -= 50.0;
            if transform.translation.distance(tempTransform.translation) < 30.0 {

                eventoExplosao.send(Explosion(Vec2::new(transform2.translation.x, transform2.translation.y) ));

                commands.entity(entidade2).despawn();
                ouro.0 += 20;
              
            }
        }
    }
}


//aqui é o sistema que verifica se a carta esta em cima de um slot de plantio, caso esteja este muda de cor
fn SlotDetectaCartas (
    commands: Commands,
    mut query: Query<(&Transform,&Dragging)>,
    mut query2: Query<(Entity, &Transform, &mut Sprite,&Slot, &mut Selecionado), Without<Planta>>,
    mut planta_selecionada: Query<(Entity,&Transform, &mut Sprite,  &mut Selecionado, ), (With <Planta>, Without<PlantaMorta>)>,
) {
    
    for (transform, _ ) in query.iter_mut() {
        for (entidade, transform2, mut sprite2, mut selecionado) in planta_selecionada.iter_mut() {
            let mut transformtemp = transform2.clone();
            transformtemp.translation.y -= 70.0;
            transformtemp.translation.x += 50.0;
            if transform.translation.distance (transformtemp.translation) < 30.0{
                sprite2.color = Color::rgb(0.1, 0.6, 0.1);
                selecionado.0 = true;
                return; // acho que isso resolve o problema de plantar sobre um slot com planta - NAO RESOLVEU
            }else{
                sprite2.color = Color::WHITE;
                selecionado.0 = false;
            }
        }
    }


    for (transform,  _ ) in query.iter_mut() {
        for (entidade, transform2, mut sprite2, _ , mut selecionado) in query2.iter_mut() {
            let mut transformtemp = transform2.clone();
            transformtemp.translation.y -= 70.0;
            transformtemp.translation.x += 50.0;
            if transform.translation.distance (transformtemp.translation) < 30.0{
                sprite2.color = Color::rgb(0.1, 0.6, 0.1);
                selecionado.0 = true;
            }else{
                sprite2.color = Color::WHITE;
                selecionado.0 = false;
            }
        }
    }
}

fn Limpa(
    mut commands: Commands,
    query: Query<(Entity, &Usada)>,
    slot: Query<(Entity, &Slot)>,
) {
    for (entidade, _) in query.iter() {
        
        commands.entity(entidade).despawn();
        
    }
    for (entidade, _) in slot.iter() {
        
        //commands.entity(entidade).despawn();
        
    }
}
//TODO: FAZER UM SISTEMA QUE COMUNICA AO JOGADOR COISAS, COMO "VOCE DEFECOU" OU "VOCE PLANTOU UMA SEMENTE"
//O TEXTO DEVE APARECER NO MEIO DA TELA, PERMANECER POR 500MS E SUMIR

//fazer 



fn Grita (
    mut eventos: EventReader<eGrita>,
    mut eventos2: EventWriter<eSpawnaGuano>,
    mut eFimTween: EventReader<TweenCompleted>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    query: Query<(Entity, &Grito)>,

)
{

    for efimTween in eFimTween.read() {
           // assert_eq!(efimTween.user_data, 42);
            if efimTween.user_data == 42 {
                eventos2.send(eSpawnaGuano());
                commands.entity(efimTween.entity).despawn();
            }
          
        
    }



    

    for _ in eventos.read() {

        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 0., LAYER_4),
                end:Vec3::new(0., 100., LAYER_4),
            }
        ).with_completed_event (42);     

        commands.spawn((
            Animator::new(tween),
            Grito,
            Text2dBundle {
            text: Text::from_section(
            "VOCE DEFECOU !",
            TextStyle {
            color: Color::WHITE,
            font: asset_server.load("Font/PressStart2P.ttf"),
            ..default()
            },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, LAYER_4)),
            ..Default::default()
        }));

    }
}



fn Fertilizar(
    event: Listener<Pointer<Up>>,//target é a carta que foi solta
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    carta: Query<(Entity, &Transform, &TipoCarta, &Dragging)>,
    mut planta_selecionada: Query<(Entity,&Transform, &mut Sprite,  &mut Selecionado, &mut Planta)>,
)
{
    //vai pegar a planta selecionada, e vai aplicar o adubo nela
    for (entidade, transform2, sprite2, selecionado, estaPlantar) in planta_selecionada.iter_mut() {
        if !selecionado.0 {
            continue;
        }

        let id2 = commands.spawn ((
            SlotGuanoFlag,
            SpriteBundle {

                texture: asset_server.load("Hectares/Cards/Slot - Guano.png"),
                sprite: Sprite{
                    
                    custom_size: Some(Vec2::new(41.0, 35.0)),
                    ..default()
                },
                transform: Transform::from_xyz(transform2.translation.x+27., transform2.translation.y-27., LAYER_2_1),
                ..default()
            },
            PickableBundle::default(),
        )).id();

        commands.entity(entidade).insert(SlotGuano(id2));
        commands.entity(event.target).insert(Usada);

    }

}


//BUG: QUANDO VC REGA , ELE VERIFICA SE EXISTE ALGUMA PLANTA E APLICA A REGA EM TODAS AS PLANTAS

fn Regar(
    event: Listener<Pointer<Up>>,//target é a carta que foi solta
    mut commands: Commands,
    carta: Query<(Entity, &Transform, &TipoCarta, &Dragging)>,
    mut planta_selecionada: Query<(Entity,&Transform, &mut Sprite,  &mut Selecionado, &mut Planta)>,
)
{

    for (entidade, transform2, mut sprite2, mut selecionado, mut estaPlantar) in planta_selecionada.iter_mut() {
        if !selecionado.0 {
            continue;
        }
        estaPlantar.1 = match estaPlantar.1 {   //muda o estado da rega da planta
            EstadoRegaPlanta::BemMolhada => EstadoRegaPlanta::BemMolhada,
            EstadoRegaPlanta::Molhada => EstadoRegaPlanta::BemMolhada,
            EstadoRegaPlanta::Normal => EstadoRegaPlanta::BemMolhada,
            EstadoRegaPlanta::QuaseSeca => EstadoRegaPlanta::BemMolhada,
            EstadoRegaPlanta::Seca => EstadoRegaPlanta::BemMolhada,
            EstadoRegaPlanta::Stressada => EstadoRegaPlanta::BemMolhada,
        };
        
        sprite2.color = Color::WHITE;
        selecionado.0 = false;
        commands.entity(event.target).insert(Usada);
    }
    //marca a carta como usada
}

fn Vender(
    event: Listener<Pointer<Up>>,//target é a carta que foi solta
    //mut eventVender: EventReader<eVender>,
    mut  commands: Commands,
    mut query2: Query<(Entity, &Transform, &Dragging, &TipoPlanta)>,
)
{

    //for _ in eventVender.read() {
     
    info!("vendeu");
        for (entidade, transform2, _, tipoplanta) in query2.iter_mut() {

        
            //    commands.entity(entidade).despawn();
                commands.entity(event.target).insert(Usada);
                info!("vendeu");

        }
    //}
}

//ao clicar no ninho, despawna todos os Mel e adiciona em ouro
fn ColheMel(
    event: Listener<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &Ninho)>,
    mut query2: Query<(Entity, &Transform, &Mel)>,
    mut ouro: ResMut<Gold>,
)
{
    for (entidade, transform, _) in query.iter_mut() {
        for (entidade2, _, _) in query2.iter_mut() {
            commands.entity(entidade2).despawn();
            ouro.0 += 10;

        }
    }
}





// não é planta, aqui vai ser o sistema de baixar uma carta em um slot, 
//se for do tipo planta ele vai plantar
fn Plantar (
    event: Listener<Pointer<Up>>,
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    mut query: Query<(Entity, &Selecionado, &Transform), (Without<Semeado>, Without<Cosmetico>, Without<Ninho>, With<Slot>) >, //slot
    mut query2: Query<(Entity, &mut Transform, &Dragging, &TipoPlanta), (With<TipoCarta>, Without<Slot>)>, //carta com o tipo de planta
)
{//aqui sera necessario tratar diferentes dipos de sementes, por eqto é só uma
    for(_,mut transformcarta,_,tipoplanta) in query2.iter_mut(){
        transformcarta.translation.z = LAYER_6;
    for (entidade, selecionado, transform) in query.iter_mut() {
        if selecionado.0 {
            
            let estruturamaturacao = match tipoplanta.0 {
                TipoPlantas::CouveRosa => EstruturaMaturacao {
                    semente: 1,
                    pequena: 2,
                    media: 2,
                    adulta: 3,
                    pronta: 1,
                },
                TipoPlantas::Hedrazeba => EstruturaMaturacao {
                    semente: 1,
                    pequena: 2,
                    media: 2,
                    adulta: 3,
                    pronta: 1,
                },
                TipoPlantas::GargomiloMiudo => EstruturaMaturacao {
                    semente: 1,
                    pequena: 2,
                    media: 2,
                    adulta: 3,
                    pronta: 1,
                },
                TipoPlantas::FolhaGorda => EstruturaMaturacao {
                    semente: 1,
                    pequena: 2,
                    media: 2,
                    adulta: 3,
                    pronta: 1,
                },
                TipoPlantas::CenouraPimenta => EstruturaMaturacao {
                    semente: 1,
                    pequena: 2,
                    media: 2,
                    adulta: 3,
                    pronta: 1,
                },
                TipoPlantas::FlorAmarela => EstruturaMaturacao {
                    semente: 1,
                    pequena: 2,
                    media: 2,
                    adulta: 3,
                    pronta: 99,
                },
            };
            
            let id = commands.spawn ((
                
                SpriteBundle {
                    texture: asset_server.load("Hectares/Cards/Slot - Agua - Normal.png"),
                    sprite: Sprite{
                    
                        custom_size: Some(Vec2::new(18.0, 20.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(transform.translation.x+27., transform.translation.y+27., LAYER_2_1),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Click>>::run(ClicaPlanta), // quando clica na planta faz isso 
            )).id();
            

            
            commands.spawn ((
                SlotAgua(id),
                tipoplanta.clone(),
                estruturamaturacao,
                MarcadorAgua(0),
                MaturacaoAtual(0),
                Planta(EstadoPlanta::Semente, EstadoRegaPlanta::BemMolhada),
                Selecionado(false),
                RefSlot(entidade),
                SpriteBundle {
                    //ta sempre carregando a mesma textura, tem que mudar isso !!!!!!!!!!!!!!!!!!!!!!
                    texture: asset_server.load("Hectares/Cards/Plantas/Couve Flor/0.png"),
                    sprite: Sprite{
                        
                        custom_size: Some(Vec2::new(101.0, 101.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(transform.translation.x-7., transform.translation.y+7., LAYER_2),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Click>>::run(ClicaPlanta), // quando clica na planta faz isso 
            ));


                    
                   // let planta = &planta.id();
                    commands.entity(entidade).insert(Semeado); //no slot
                    commands.entity(event.target).insert(Usada); //na carta
                      
            }
        }}  
}

//uma função que varre as plantas e ve se ela tem alguma perto, se tiver o slot dela fica roxo
fn DetectaSimbiose (
    commands: Commands,
    mut query: Query<(Entity, &Transform, &Planta)>,
    mut query2: Query<(Entity, &Transform, &mut Sprite, &Planta)>,
)
{
    for (entidade, transform, _ ) in query.iter_mut() {
        for (entidade2, transform2, mut sprite2, _ ) in query2.iter_mut() {
            if transform.translation.distance(transform2.translation) < 200.0{
                sprite2.color = Color::rgb(1.3, 1.1, 1.3);
            }else{ 
                sprite2.color = Color::WHITE;
            }
        }
    }
}





fn ClicaPlanta (
    event: Listener<Pointer<Click>>,
    mut eventos: EventWriter<ColheuPlanta>,
    mut commands: Commands,
    mut query: Query<(Entity,&Transform, &Planta,&TipoPlanta ,  &mut SlotAgua, &mut RefSlot)>,
)
{
    
    for (entidade, transform, estaPlantar, tipoplanta ,  idSlot, refslot ) in query.get_mut(event.target) {
        
        match estaPlantar.0 {
            EstadoPlanta::Semente => {
                info!("estado da planta semente");
                
                return;
            }

            EstadoPlanta::Pequena => {
                info!("estado da planta Pequena");
            }
            EstadoPlanta::Media => {
                info!("estado da planta Media");
            }
            EstadoPlanta::Adulta => {
                info!("estado da planta Adulta");
            }
            EstadoPlanta::Pronta => {
                info!("estado da planta Pronta");
                eventos.send(ColheuPlanta(tipoplanta.0));
                eventos.send(ColheuPlanta(tipoplanta.0));
                
                commands.entity(idSlot.0).despawn();
                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    std::time::Duration::from_secs(1),
                    TransformPositionLens{
                        start:Vec3::new(transform.translation.x, transform.translation.y , LAYER_4),
                        end:Vec3::new(-300., 300., LAYER_4),
                    }
                ).with_completed_event (42);  

               commands.entity(entidade).insert(Animator::new(tween)); 
               // commands.entity(entidade).despawn(); 
                info!("COLHEU PLANTA PRONTA !");
                return;


            }
        }
        match estaPlantar.1 {
            EstadoRegaPlanta::BemMolhada => {
                info!("estado da planta BemMolhada");
            }
            EstadoRegaPlanta::Molhada => {
                info!("estado da planta Molhada");
            }
            EstadoRegaPlanta::Normal => {
                info!("estado da planta Normal");
            }
            EstadoRegaPlanta::QuaseSeca => {
                info!("estado da planta QuaseSeca");
            }
            EstadoRegaPlanta::Seca => {
                info!("estado da planta Seca");
            }
            EstadoRegaPlanta::Stressada => {
                info!("estado da planta Stressada");
            }
        }

        //info!("estado da planta {:#?}", estaPlantar);

          //  commands.entity(entidade).insert(Semeado);
        
    }
}





fn AtualizaGuano(
    mut query: Query<(Entity, &mut Transform, &SlotGuano)>,
    query2: Query<(Entity, &Transform, &mut Sprite, &SlotGuano)>,
)
{
    for (entidade, transform, _ ) in query.iter_mut() {

        }
}

fn SpawnaCosmetico (
    mut events: EventReader<eSpawnaCosmetico>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut query: Query<(Entity, &Transform, &Slot), (Without<Cosmetico>, Without<Semeado>)>,
)
{
    for _ in events.read() {
        let mut rng = thread_rng();
        let coin = rng.gen_range(0 .. 30);
        let mut temp = 0;

        for (entidade, transformz,_) in query.iter_mut()
        {
         

            if coin == temp {
                let mut transf = transformz.clone();
                transf.translation.z = LAYER_2;
                let coin2 = rng.gen_range(0 .. 8);    
                let textura = match coin2 {

                    0 => asset_server.load("Hectares/Cards/Arbusto.png"),
                    1 => asset_server.load("Hectares/Cards/Arbusto2.png"),
                    2 => asset_server.load("Hectares/Cards/Plantas/Gorgo.png"),
                    3 => asset_server.load("Hectares/Cards/Mato2.png"),
                    4 => asset_server.load("Hectares/Cards/Plantas/Gorgo.png"),
                    5 => asset_server.load("Hectares/Cards/Flor1.png"),
                    6 => asset_server.load("Hectares/Cards/Flor1.png"),
                    7 => asset_server.load("Hectares/Cards/Flor2.png"),
                    8 => asset_server.load("Hectares/Cards/Flor2.png"),
                    _ => asset_server.load("Hectares/Cards/Flor2.png"),
                };
                
                let mut temp = commands.spawn((

                   

                    
                    CosmeticoFlag,
                    Cosmetico,
                    SpriteBundle {

                        texture:textura,


                        sprite: Sprite{
                        
                            custom_size: Some(Vec2::new(70.0, 70.0)),
                            ..default()
                        },
                        transform: transf ,
                        ..default()
                    },
                    PickableBundle::default(), 
                ));
                if coin2 == 5 || coin2 == 6 || coin2 == 7 || coin2 == 8 {
                    temp.insert(FlorAmarela);
                }
                commands.entity(entidade).insert(Cosmetico);
                commands.entity(entidade).insert(Semeado);
                return;
                
            }
            
            temp += 1;

        }
        info!("nao espawnow nada");
    }
    
}



fn SpawnaGuano(
    mut events: EventReader<eSpawnaGuano>,
    mut eventos_semente: EventReader<eSpawnaSemente>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
)
{
    for _ in events.read()  {
        let mut rng = thread_rng();
        let x = rng.gen_range(-100.0..100.0);
            
            let coin = rng.gen_range(0 .. 2);

            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_secs(1),
                TransformPositionLens{
                    start:Vec3::new(0., 0., LAYER_3_1),
                    end:Vec3::new(x, -350. + x, LAYER_3_1),
                }
            );
        
            commands.spawn((

                TipoCarta(TiposCarta::Guano),
                Animator::new(tween),                                               
                SpriteBundle {
                    texture: asset_server.load("Hectares/Cards/Recurso - Guano.png"),
                    sprite: Sprite{
                    
                        custom_size: Some(Vec2::new(113.0, 154.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., LAYER_3_1),
                    ..default()
                },
                PickableBundle::default(), // <- Makes the mesh pickable.
                On::<Pointer<DragStart>>::target_insert(Dragging), // <- Add the Dragging component to the entity    
                On::<Pointer<DragEnd>>::target_remove::<Dragging>(), // <- Remove the Dragging component from the entity
                On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                    transform.translation.x += drag.delta.x; // Make the square follow the mouse
                    transform.translation.y -= drag.delta.y;
                    transform.translation.z = LAYER_6;

                }),
                On::<Pointer<Up>>::run(Fertilizar),

            ));
    }
    for event in eventos_semente.read()  {
        let mut rng = thread_rng();
        let x = rng.gen_range(-100.0..100.0);

        let tipoplanta = event.0;

        let texturacarta = match tipoplanta {
            TipoPlantas::CouveRosa => "Hectares/Cards/Plantas/Couve Flor/Carta.png",
            TipoPlantas::Hedrazeba => "Hectares/Cards/Plantas/Hedrazeba/Carta.png",
            TipoPlantas::GargomiloMiudo => "Hectares/Cards/Plantas/Gargomilo Miudo/Carta.png",
            TipoPlantas::FolhaGorda => "Hectares/Cards/Plantas/Folha Gorda/Carta.png",
            TipoPlantas::CenouraPimenta => "Hectares/Cards/Plantas/Cenoura Pimenta/Carta.png",
            TipoPlantas::FlorAmarela => "Hectares/Cards/Plantas/Flor Amarela/Carta.png",
        };

            
            let coin = rng.gen_range(0 .. 2);
    
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_secs(1),
                TransformPositionLens{
                    start:Vec3::new(0., 0., LAYER_3_1),
                    end:Vec3::new(x, -350. + x, LAYER_3_1),
                }
            );
        
            commands.spawn((
    
                TipoCarta(TiposCarta::Semente),
                TipoPlanta(tipoplanta),
                Animator::new(tween),                                               
                SpriteBundle {
                    texture: asset_server.load(texturacarta),
                    sprite: Sprite{
                    
                        custom_size: Some(Vec2::new(113.0, 154.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., LAYER_3_1),
                    ..default()
                },
                PickableBundle::default(), // <- Makes the mesh pickable.
                On::<Pointer<DragStart>>::target_insert(Dragging), // <- Add the Dragging component to the entity    
                On::<Pointer<DragEnd>>::target_remove::<Dragging>(), // <- Remove the Dragging component from the entity
                On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                    transform.translation.x += drag.delta.x; // Make the square follow the mouse
                    transform.translation.y -= drag.delta.y;

                }),
                On::<Pointer<Up>>::run(Plantar),
                //tipo_carta(TipoCarta::Agua),
            ));
    }
}


/*
fn SpawnaSemente(

    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
)
{



    let mut rng = thread_rng();
    let x = rng.gen_range(-100.0..100.0);
        
        let coin = rng.gen_range(0 .. 2);
        let coinTipoPlanta = rng.gen_range(0 .. 4);

        let TipoPlantaTemp = match coinTipoPlanta {
            0 => TipoPlantas::CouveRosa,
            1 => TipoPlantas::Hedrazeba,
            2 => TipoPlantas::GargomiloMiudo,
            3 => TipoPlantas::FolhaGorda,
            _ => TipoPlantas::CenouraPimenta,
        };
        let TexturaPlantaTemp = match TipoPlantaTemp {
            TipoPlantas::CouveRosa => "Hectares/Cards/Plantas/Couve Flor/Carta.png",
            TipoPlantas::Hedrazeba => "Hectares/Cards/Plantas/Hedrazeba/Carta.png",
            TipoPlantas::GargomiloMiudo => "Hectares/Cards/Plantas/Gargomilo Miudo/Carta.png",
            TipoPlantas::FolhaGorda => "Hectares/Cards/Plantas/Folha Gorda/Carta.png",
            TipoPlantas::CenouraPimenta => "Hectares/Cards/Plantas/Cenoura Pimenta/Carta.png",
        };

        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 0., 0.5),
                end:Vec3::new(x, -350. + x, 0.5),
            }
        );
    
        commands.spawn((

            TipoCarta(TiposCarta::Semente),
            TipoPlanta(TipoPlantaTemp),
            Animator::new(tween),                                               
            SpriteBundle {
                texture: asset_server.load(TexturaPlantaTemp),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(113.0, 154.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            }
            //tipo_carta(TipoCarta::Agua),
        ));
}*/

//fn DespawnaPlantas(

fn DespawnaMercado ( //isso despawna tudo na verdad
    mut commands: Commands,
    mut eFimTween: EventReader<TweenCompleted>,
    mut eventoExplosao: EventWriter<Explosion>,
)
{
    for efimTween in eFimTween.read() {
        if efimTween.user_data == 77 {
            eventoExplosao.send(Explosion(Vec2::new(-135.,390. ) ));
            commands.entity(efimTween.entity).despawn();
        }
        if efimTween.user_data == 66 {
            
            commands.entity(efimTween.entity).despawn();
        }
    }
}   

fn FechaMercado (
    mut commands: Commands,
    query: Query<(Entity, &Mercado)>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,
)
{

    for (entidade, _) in query.iter() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 0., LAYER_4),
                end:Vec3::new(-500., 0., LAYER_4),
            }
        ).with_completed_event (66);
        app_state.set(AppState::InGame);
        commands.entity(entidade).insert(Animator::new(tween));
        
        //commands.entity(entidade).despawn();
    }
}
fn AbreDialogo(
    mut eabredialogo : EventReader<eAbreDialogo>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Dialogo)>,
    asset_server: ResMut<AssetServer>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,
 )
 {
    for (_, mut transform, _) in query.iter_mut(){
        let time = time.elapsed_seconds() as f32;
        transform.scale.x  += (time * 5.0).sin() /DIALOGO_VEL_X ;
        transform.scale.y  -= (time * 5.0).sin() /DIALOGO_VEL_Y ;
    }

    for _ in eabredialogo.read() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., -600., LAYER_4),
                end:Vec3::new(0., -300., LAYER_4),
            }
        );
        
        commands.spawn((
            Resultados,
            Dialogo,
            Animator::new(tween),
            SpriteBundle {
                texture: asset_server.load("Hectares/Cards/Misc/Emissaria1.png"),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(454.0, 193.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., LAYER_4),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(FechaResultados),
        ));
        app_state.set(AppState::Paused);

    }

 }

 fn AbreResultados(
    mut eabreresultados : EventReader<eAbreResultados>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,
 )
 {
    for _ in eabreresultados.read() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 0., LAYER_4),
                end:Vec3::new(0., 0., LAYER_4),
            }
        );
        
        commands.spawn((

            Resultados,
            Animator::new(tween),
            SpriteBundle {
                texture: asset_server.load("Hectares/Cards/Resultados/Resultados Teste.png"),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(406.0, 708.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., LAYER_4),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(FechaResultados),
        ));
        app_state.set(AppState::Paused);

    }

 }
fn FechaResultados (
    mut commands: Commands,
    query: Query<(Entity, &Resultados)>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,
)
{

    for (entidade, _) in query.iter() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 0., LAYER_4),
                end:Vec3::new(-500., 0., LAYER_4),
            }
        ).with_completed_event (66);
        app_state.set(AppState::InGame);
        commands.entity(entidade).insert(Animator::new(tween));
        //commands.entity(entidade).despawn();
    }
}



//uma funcao que faça ao clicar numa planta do inventario, 
//diminua a quantidade dela no inventario e dropa 2 cartas 
//de semente do tipo dela

fn TransformaPlantaProntaSemente(
    event: Listener<Pointer<Click>>,//aqui vc consegue pegar a planta que foi clicada e usar o tipo dela para fazer a semente
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &ItemInventarioUI)>, //aqui vc pega a planta que foi clicada
    mut eventos: EventWriter<eSpawnaSemente>, //ele precisa spawnar duas sementes do tipo da planta clicada
    mut eventos2: EventWriter<eSubtraiUmaPlantaInventario>, //ele precisa subtrair uma planta do inventario
)
{
    //nao precisa usar tween, vc só chama os eventos de spawnar semente e subtrair uma planta do inventario
    //usar event target para pegar a planta clicada
    for (entidade, transform, item) in query.get_mut(event.target) {
        eventos.send(eSpawnaSemente(item.0));
        eventos2.send(eSubtraiUmaPlantaInventario(item.0));
    }


}

//isso faz aparecer uma planta no seu inventario
fn AdicionaRemoveUmaPlanta (
    mut commands: Commands,
    mut evento_adiciona_uma_planta_inventario: EventReader<eAdicionaUmaPlantaInventario>,
    mut evento_subtrai_uma_planta_inventario: EventReader<eSubtraiUmaPlantaInventario>,
    mut itens_inventario: Option<ResMut<Inventario>>,

)
{
    for evento in evento_adiciona_uma_planta_inventario.read() {
        
        let tipoplanta = &evento.0;

        for mut item in itens_inventario.as_mut().unwrap().itens.iter_mut(){
            if item.tipo == *tipoplanta && item.visivel == true {
                item.quantidade += 1;
                return;
            }

        }
        
        /*itens_inventario.unwrap().itens.push(ItemInventario{
            tipo: TipoPlantas::CouveRosa,
            quantidade: 1,
            visivel: false,
        });
        return;*/

    }
    for evento2 in evento_subtrai_uma_planta_inventario.read() {
        info!("Subtrai uma planta");
        let tipoplanta = &evento2.0;

        for mut item in itens_inventario.as_mut().unwrap().itens.iter_mut(){
            if item.tipo == *tipoplanta && item.visivel == true {

                if item.quantidade == 0 {
                    return;
                }
                item.quantidade -= 1;
                
                return;
            }

        }


    }


}



//as plantas aparecem no canto da tela com o icone dela e a quantidade ao lado em baixo
//issso aparece na tela do jogo mesmo

fn AtualizaInventario(
    mut commands: Commands, 
    asset_server: ResMut<AssetServer>,
    mut itens_inventario: Option<ResMut<Inventario>>,   
    mut query: Query<(Entity, &mut Text, &TextoItemInventarioUI)>,
)
{
    let mut num_item = 0;
    for mut item in itens_inventario.unwrap().itens.iter_mut(){
        if item.visivel == true {
            num_item += 1;

            for (entidade, mut texto, item_tipo) in query.iter_mut() {

                if item_tipo.0 == item.tipo {
                    texto.sections[0].value = item.quantidade.to_string();
                }
                
            }

            continue;
        }
        println!("Item: {:#?}, Quantidade: {:#?}", item.tipo, item.quantidade);
        item.visivel = true;    
        //spawna uma imagem da planta pronta no lado direito da tela com a quantidade

        let textura = match item.tipo {
            TipoPlantas::CouveRosa => "Hectares/Cards/Plantas/Couve Flor/5.png",
            TipoPlantas::Hedrazeba => "Hectares/Cards/Plantas/Hedrazeba/5.png",
            TipoPlantas::GargomiloMiudo => "Hectares/Cards/Plantas/Gargomilo Miudo/5.png",
            TipoPlantas::FolhaGorda => "Hectares/Cards/Plantas/Folha Gorda/5.png",
            TipoPlantas::CenouraPimenta => "Hectares/Cards/Plantas/Cenoura Pimenta/5.png",
            TipoPlantas::FlorAmarela => "Hectares/Cards/Plantas/Flor Amarela/5.png",
        };
        //não precisa de tween
        commands.spawn((

            ItemInventarioUI(item.tipo),
            SpriteBundle {
                texture: asset_server.load(textura),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(90.0, 90.0)),
                    ..default()
                },
                transform: Transform::from_xyz(220., 300. - ( num_item as f32 * 60.0 ) , LAYER_3),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(TransformaPlantaProntaSemente),
        ));
        let qtd = item.quantidade;
        commands.spawn((
            TextoItemInventarioUI(item.tipo),
            Text2dBundle {
                transform: Transform::from_translation(Vec3::new(235., 310. - ( num_item as f32 * 60.0 ), LAYER_3_0)),
                text: Text::from_section(
                    qtd.to_string(),
                    TextStyle {
                        color: Color::WHITE,
                        font: asset_server.load("Font/PressStart2P.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                ),
                
                ..Default::default()
            })
        );
        num_item += 1;
    }

   
}


fn AbreTransmutador(
    mut evento_adiciona_uma_planta_inventario: EventWriter<eSubtraiUmaPlantaInventario>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,
 )
 {


    evento_adiciona_uma_planta_inventario.send(eSubtraiUmaPlantaInventario(TipoPlantas::CouveRosa));
    /*
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 500., LAYER_4),
                end:Vec3::new(0., 0., LAYER_4),
            }
        );
        
        commands.spawn((

            Resultados,
            Animator::new(tween),
            SpriteBundle {
                texture: asset_server.load("Hectares/Cards/Transmutador/Tela.png"),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(412.0, 109.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., LAYER_4),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(FechaTransmutador),
        ));
        app_state.set(AppState::Paused);

    
*/
 }
fn FechaTransmutador (
    mut commands: Commands,
    query: Query<(Entity, &Resultados)>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>,
)
{

    for (entidade, _) in query.iter() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 0., LAYER_4),
                end:Vec3::new(500., 0., LAYER_4),
            }
        ).with_completed_event (66);
        app_state.set(AppState::InGame);
        commands.entity(entidade).insert(Animator::new(tween));
        //commands.entity(entidade).despawn();
    }
}


fn AbreMensagem(
    mut eabremensagem : EventReader<eAbreMensagem>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>
)
{
    for _ in eabremensagem.read() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., -500., LAYER_4),
                end:Vec3::new(0., 0., LAYER_4),
            }
        );

        commands.spawn((
            Mensagem,
            Animator::new(tween),
            SpriteBundle {
                texture: asset_server.load("./Hectares/Cards/Misc/Mensagem.png"),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(458.0, 189.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., LAYER_4),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(FechaMensagem),
        ));
       // app_state.set(AppState::Paused);
    }

}

fn FechaMensagem (
    mut commands: Commands,
    query: Query<(Entity, &Mensagem)>,
    app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>
)
{

    for (entidade, _) in query.iter() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 0.,LAYER_4 ),
                end:Vec3::new(0., 500., LAYER_4),
            }
        ).with_completed_event (66);
        commands.entity(entidade).insert(Animator::new(tween));
      //  app_state.set(AppState::InGame);
        //commands.entity(entidade).despawn();
    }
}


 


fn AbrePedidos(
    mut eabrepedidos : EventReader<eAbrePedidos>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>
)
{
    for _ in eabrepedidos.read() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(220., 0., LAYER_4),
                end:Vec3::new(0., 0., LAYER_4),
            }
        );

        //lista 3 pedidos
        for i in 0 .. 3 {
            let tween3 = Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_secs(1),
                TransformPositionLens{
                    start:Vec3::new(220., 0. + (i as f32 * 40.), LAYER_5),
                    end:Vec3::new(0., 0. + (i as f32 * 40.), LAYER_5),
                });
                let tween4 = Tween::new(
                    EaseFunction::QuadraticInOut,
                    std::time::Duration::from_secs(1),
                    TransformPositionLens{
                        start:Vec3::new(220., 0. + (i as f32 * 40.), LAYER_4),
                        end:Vec3::new(0., 0. + (i as f32 * 40.), LAYER_4),
                    });
            
        
            
            let tempid = commands.spawn(
    
                (         
                    Pedidos,           
                    Animator::new(tween3),
                Text2dBundle {
                
                text: Text::from_section(
                "2X Couve Rosa 50z",
                TextStyle {
                color: Color::BLACK,
                font: asset_server.load("./Font/PressStart2P.ttf"),
                font_size: 20.0,
                ..default()
                },
                ),
                transform: Transform::from_translation(Vec3::new(30.0, 400.0, LAYER_5)),
                ..Default::default()
                },
                Pickable::IGNORE, 
            )).id();

            commands.spawn((
                ItemPedido{
                    tipo: TipoPlantas::CouveRosa,
                    quantidade: 2,
                    valor: 50,
                },
                RefTextoPedido(tempid),
                Pedidos,
                Animator::new(tween4),
                SpriteBundle {
                    sprite: Sprite{
                        color: Color::hsla(0.6, 0.8, 0.6, 0.8),
                        custom_size: Some(Vec2::new(370.0, 38.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(200., 0. + (i as f32 * 40.), LAYER_6),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Click>>::run(ResolvePedidos),
            ));

        }


        commands.spawn((
            Pedidos,
            Animator::new(tween),
            SpriteBundle {
                texture: asset_server.load("./Hectares/Cards/Pedidos/Pedidos.png"),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(406.0, 708.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., LAYER_4),
                ..default()
            },
            //Pickable::IGNORE, /// AQUI PODE FECHAR TB
            PickableBundle::default(),
            On::<Pointer<Click>>::run(FechaPedidos),
            //On::<Pointer<Click>>::run(ResolvePedidos),
        ));

        app_state.set(AppState::Paused);
    }

}

fn ResolvePedidos(
    mut event: Listener<Pointer<Click>>,//no target vc pega o pedido clicado
    mut commands: Commands,
    mut query: Query<(Entity, &ItemPedido, &Pedidos, &RefTextoPedido)>,//aqui pega o item pedido e cruz o entity com o target usando get_mut()
    mut inventario: ResMut<Inventario>,
    mut gold: ResMut<Gold>,
)
{


    //precisa pegar o evento click pra ver em qual pedido foi clicado, cruzar com uma query dos pedidos e pegar o item do pedido
        info!("Pedido Clicado: {:#?}", event.target);
 
        
    for (entidade, item, _, reftextopedido) in query.get_mut(event.target) {
        info!("Item Pedido: {:#?}", item);

        for mut item_inventario in inventario.itens.iter_mut(){
            if item_inventario.tipo == item.tipo {
                if item_inventario.quantidade < item.quantidade {
                    info!("Não tem quantidade suficiente");
                    return;
                }
                item_inventario.quantidade -= item.quantidade;
                commands.entity(entidade).despawn();
                commands.entity(reftextopedido.0).despawn();
                gold.0 += item.valor;
                info!("Pedido atendido: {:#?}", item);
                return;
            }
        }


        
    }
        //aqui vc faz o que quiser com o item do pedido
     //   info!("Item Pedido: {:#?}", item);
        //commands.entity(event.target).despawn();
      //  commands.entity(event.target.RefTextoPedido.0).despawn();
   // }


}

fn FechaPedidos (
    mut commands: Commands,
    query: Query<(Entity, &Pedidos, &mut Transform)>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>
)
{

    for (entidade, _, mut transform) in query.iter() {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(transform.translation.x, transform.translation.y, LAYER_4),
                end:Vec3::new(transform.translation.x -500., transform.translation.y, LAYER_4),
            }
        ).with_completed_event (66);
        commands.entity(entidade).insert(Animator::new(tween));
        app_state.set(AppState::InGame);
        //commands.entity(entidade).despawn();
    }
}

//TODO: FAZER COM QUE AO FECHAR E ABRIR O BOTAO FICA DESABILITADO ATE ACABAR O TWEEN

fn AbreMercado (
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    query: Query<(Entity, &Mercado)>,
    query2: Query<(Entity, &Relogio)>,
    mut app_state: ResMut<NextState<AppState>>, 
    app_state_atual: Res<State<AppState>>
)
{
    let mut pode = true;

    for (_, relogio) in query2.iter() {
        if relogio.0 != FaseDia::MeioManha {
            pode = false;
        }
    }

    for (entidade, _) in query.iter() {
        let tween = Tween::new(
            EaseFunction::BounceOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(0., 0.,LAYER_4),
                end:Vec3::new(-500., 0., LAYER_4),
            }
        ).with_completed_event (66);
        commands.entity(entidade).insert(Animator::new(tween));
        app_state.set(AppState::InGame);
        return;
    }
 
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(-500., 0., LAYER_4),
                end:Vec3::new(0., 0.,LAYER_4),
            }
        );

        let temptextura = match pode {
            true => "./Hectares/Cards/Mercado/Aberto.png",
            false => "./Hectares/Cards/Mercado/Fechado.png",
        };

        commands.spawn((
            Mercado,
            Animator::new(tween),
            SpriteBundle {
                texture: asset_server.load(temptextura),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(406.0, 708.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., LAYER_4),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(FechaMercado),
        ));

    //coloca se 4 cartas, 2 em cima 2 em baixo no meio da tela, a imagem da carta que pode ser de semente de alguma planta monta 4 randomicas
    //ao clicar na carta ela some e aparece uma carta de semente da planta da carta clicada
    for i in 0 .. 2 {
        for n in 0 .. 2 {
            let mut rng = thread_rng();
            let x = -100 + (i * 200);
            let y = 200 - (n * 400);
            let coin = rng.gen_range(0 .. 2);
            let coinTipoPlanta = rng.gen_range(0 .. 4);
    
            let TipoPlantaTemp = match coinTipoPlanta {
                0 => TipoPlantas::CouveRosa,
                1 => TipoPlantas::Hedrazeba,
                2 => TipoPlantas::GargomiloMiudo,
                3 => TipoPlantas::FolhaGorda,
                _ => TipoPlantas::CenouraPimenta,
            };
            let TexturaPlantaTemp = match TipoPlantaTemp {
                TipoPlantas::CouveRosa => "./Hectares/Cards/Plantas/Couve Flor/Carta.png",
                TipoPlantas::Hedrazeba => "./Hectares/Cards/Plantas/Hedrazeba/Carta.png",
                TipoPlantas::GargomiloMiudo => "./Hectares/Cards/Plantas/Gargomilo Miudo/Carta.png",
                TipoPlantas::FolhaGorda => "./Hectares/Cards/Plantas/Folha Gorda/Carta.png",
                TipoPlantas::CenouraPimenta => "./Hectares/Cards/Plantas/Cenoura Pimenta/Carta.png",
                TipoPlantas::FlorAmarela => "./Hectares/Cards/Plantas/Flor Amarela/Carta.png",
            };
    
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_secs(1),
                TransformPositionLens{
                    start:Vec3::new(0., 0., LAYER_5),
                    end:Vec3::new(x as f32, y as f32, LAYER_5),
                }
            );
        
            commands.spawn((
    
                TipoCarta(TiposCarta::Semente),
                TipoPlanta(TipoPlantaTemp),
                Animator::new(tween),                                               
                SpriteBundle {
                    texture: asset_server.load(TexturaPlantaTemp),
                    sprite: Sprite{
                    
                        custom_size: Some(Vec2::new(113.0, 154.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., LAYER_4),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Click>>::run(TransformaPlantaProntaSemente),
            ));
        }
    }



        app_state.set(AppState::Paused);
        
 
}







fn SpawnaAgua (
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
)

{
    
    let mut rng = thread_rng();
    let x = rng.gen_range(-100.0..100.0);
        
        let coin = rng.gen_range(0 .. 2);

        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_secs(1),
            TransformPositionLens{
                start:Vec3::new(100., 390., LAYER_3_1),
                end:Vec3::new(x, -350. + x, LAYER_3_1),
            }
        );
    
        commands.spawn((

            TipoCarta(TiposCarta::Agua),
            Animator::new(tween),                                               
            SpriteBundle {
                texture: asset_server.load(
                    "./Hectares/Cards/Recurso - Agua.png"),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(113.0, 154.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., LAYER_3_1),
                ..default()
            },

            PickableBundle::default(), // <- Makes the mesh pickable.
            On::<Pointer<DragStart>>::target_insert(Dragging), // <- Add the Dragging component to the entity    
            On::<Pointer<DragEnd>>::target_remove::<Dragging>(), // <- Remove the Dragging component from the entity
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;

            }),
            On::<Pointer<Up>>::run(Regar),
            
        ));
}

fn MateriaPrimaSegueAbelha(
    mut qabelha: Query<(Entity, &Transform, &RefMateriaPrima), With<Abelha>>,
    mut qmateriaprima: Query<(Entity, &mut Transform, &MateriaPrima), Without<Abelha>>,
)
{
    for (entidade2, mut transform2, refabelha) in qmateriaprima.iter_mut() {
    for (entidade, transform, refmateriaprima) in qabelha.get_mut(refabelha.0) {
        
            
                let mut tempTransform = transform.clone();
                tempTransform.translation.z = LAYER_3_0;
                tempTransform.translation.x += 20.0;
                tempTransform.translation.y += 20.0;
                tempTransform.rotation = Quat::from_rotation_z(0.0);
                transform2.translation = tempTransform.translation;
            
        }
    }    
}


fn MovimentaAbelha (
    mut qabelha: Query<(Entity, &mut Transform, &mut EstadoAbelhaAtual, &mut Target, Option<&RefMateriaPrima>), (With <Abelha>, Without<Ninho>)>,
    mut qflor: Query<(Entity, &mut Transform), (With<FlorAmarela>, Without<Abelha>, Without<Ninho>)>,
    mut qninho: Query<(Entity, &mut Transform),(With<Ninho>, Without<Abelha>, Without<FlorAmarela>)>,
    
    asset_server: ResMut<AssetServer>,
    time: Res<Time>,
    mut commands: Commands,
)
{
    let time = time.elapsed_seconds() as f32;

    let mut rng = thread_rng();

    //escolhe uma flor aleatoria e guarda num temp
    for (entidade, _) in qflor.iter() {
        let coin = rng.gen_range(0 .. 10);
        let temp = entidade;
        if coin == 0 {
            break;
        }
    }



    for (entidade, mut transform, mut estadoatual, mut target, refmateriaprima) in qabelha.iter_mut() {
    
        match estadoatual.0 {
            EstadoAbelha::IndoFlor => {
                for (entidadeninho, transform2) in qninho.iter_mut() { 
                for (entidade2, transform2) in qflor.get_mut(target.0) {
                    //anda em direção a flor
                    transform.translation.x += (transform2.translation.x - transform.translation.x) / ABELHA_VEL_X;
                    transform.translation.y += (transform2.translation.y - transform.translation.y) /ABELHA_VEL_Y;
                    transform.rotation *= Quat::from_rotation_z((time * ABELHA_ROT_MULT).sin()/ ABELHA_ROT_OFFSET);
                    transform.translation.x += (time * ABELHA_MULT_X).sin() ;
                    transform.translation.y += (time * ABELHA_MULT_Y).sin() ;
                  
                    //info!("Indo Flor {}", transform.translation.distance(transform2.translation) );
                    if transform.translation.distance(transform2.translation) <35.0 {
                        let mut tempTransform = transform.clone();
                        tempTransform.translation.z = LAYER_3_0;
                        tempTransform.translation.x += 10.0;
                        tempTransform.translation.y += 10.0;

                        estadoatual.0 = EstadoAbelha::IndoNinho;

                        target.0 = entidadeninho;
                      
                        let id = commands.spawn ((
                            MateriaPrima(entidade), // a referencia da abelha que esse sprite vai seguir
                            SpriteBundle {
                                texture: asset_server.load("./Hectares/Cards/insetos/Materia Prima.png"),
                                sprite: Sprite{     
                                    custom_size: Some(Vec2::new(30., 30.)),
                                    ..default()
                                },
                                transform: tempTransform,
                                ..default()
                            },
                            PickableBundle::default(),
                        )
                        ).id();
                        commands.entity(entidade).insert (RefMateriaPrima(id));
                        return;
                    }
                }
            }
            }  
            EstadoAbelha::IndoNinho => {
                for (entidadeflor, transform2) in qflor.iter_mut() {
                for (entidade2, transform2) in qninho.iter_mut() {
                    //anda em direção a flor
                    transform.translation.x += (transform2.translation.x - transform.translation.x) /ABELHA_VEL_X2 ;
                    transform.translation.y += (transform2.translation.y - transform.translation.y) /ABELHA_VEL_Y2 ;
                    transform.rotation *= Quat::from_rotation_z((time * ABELHA_ROT_MULT).sin()/ ABELHA_ROT_OFFSET);
                    transform.translation.x += (time * ABELHA_MULT_X).sin();
                    transform.translation.y += (time * ABELHA_MULT_Y).sin() ;

                    if transform.translation.distance(transform2.translation) < 35.0 {
                        
                        //escolhe uma nova flor !!!

                       if let Some(refmateriaprima) = refmateriaprima {
                            let mut tempTransform = transform2.clone();
                            tempTransform.translation.z = LAYER_2_1;
                            tempTransform.translation.x -= 20.0;
                            tempTransform.translation.y += 40.0;
       
                            
                            commands.entity(refmateriaprima.0).despawn();
                            commands.spawn ((
                                Mel, 
                                SpriteBundle {
                                    texture: asset_server.load("./Hectares/Cards/insetos/Mel.png"),
                                    sprite: Sprite{     
                                        custom_size: Some(Vec2::new(24., 41.)),
                                        ..default()
                                    },
                                    transform: tempTransform,
                                    ..default()
                                },
                                Pickable::IGNORE,
                            ));
                    
                     

                        }                      
                        estadoatual.0 = EstadoAbelha::IndoFlor;
                        target.0 = entidadeflor;
                    }
                }
            }
            }
            _ => {
                for (entidadeflor, transform2) in qflor.iter_mut() {
                    for (entidade2, transform2) in qninho.iter_mut() {
                        //anda em direção a flor
                        transform.translation.x += (transform2.translation.x - transform.translation.x) /ABELHA_VEL_X;
                        transform.translation.y += (transform2.translation.y - transform.translation.y) / ABELHA_VEL_Y;
                        transform.rotation *= Quat::from_rotation_z((time * ABELHA_ROT_MULT).sin()/ ABELHA_ROT_OFFSET);
                        transform.translation.x += (time * ABELHA_MULT_X).sin() ;
                        transform.translation.y += (time * ABELHA_MULT_Y).sin() ;
                        if transform.translation.distance(transform2.translation) < 30.0 {
                            info!("Roaming");
                            estadoatual.0 = EstadoAbelha::IndoFlor;
                            target.0 = entidadeflor;
                        }
                    }
                }
            }

        }

    }
}


fn MovimentaMosca (
    mut query: Query<(Entity, &mut Transform, &mut Inseto)>,
    mut query2: Query<(Entity, &mut Transform, &mut Planta),Without<Inseto>>,
 
    asset_server: ResMut<AssetServer>,
    time: Res<Time>,
    commands: Commands,
)
{
    let time = time.elapsed_seconds() as f32;

    //pega o translation de uma planta qualquer e voa até ela
    for (entidade, mut transform, inseto) in query.iter_mut() {
        for (entidade2, transform2, planta) in query2.iter_mut() {
            //anda em direção a planta
            transform.translation.x += (transform2.translation.x - transform.translation.x) / MOSCA_VEL_X;
            transform.translation.y += (transform2.translation.y - transform.translation.y) / MOSCA_VEL_Y;
            transform.rotation *= Quat::from_rotation_z((time * ABELHA_ROT_MULT).sin()/ ABELHA_ROT_OFFSET);
            transform.translation.x += (time * MOSCA_MULT_X).sin() ;
            transform.translation.y += (time * MOSCA_MULT_Y).sin() ;
            
        }
    }

    //a abelha pega o nectar e leva para o ninho


}




//nao usar mais essa, usar as especificas
fn SpawnaCarta (
    mut events: EventReader<eSpawnaCarta>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
)

{
    for _ in events.read() {
        let mut rng = thread_rng();
        let x = rng.gen_range(-200.0..200.0);
            
            let coin = rng.gen_range(0 .. 3);

            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_secs(1),
                TransformPositionLens{
                    start:Vec3::new(0., 0., 0.5),
                    end:Vec3::new(x, -350. + (x /2.4), 0.5),
                }
            );
        
            commands.spawn((
                match coin {
                    0 => TipoCarta(TiposCarta::Semente),
                    1 => TipoCarta(TiposCarta::Agua),
                    _ => TipoCarta(TiposCarta::Guano),
                },
                


                Animator::new(tween),                                               

                match coin {
                    0 => SpriteBundle {
                        texture: asset_server.load("./Hectares/Cards/Semente - Cenoura Pimenta.png"),
                        sprite: Sprite{
                        
                            custom_size: Some(Vec2::new(113.0, 154.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.,0.,0.),
                        ..default()
                    },
                    1 => SpriteBundle {
                        texture: asset_server.load(
                            "./Hectares/Cards/Recurso - Agua.png"),
                        sprite: Sprite{
                        
                            custom_size: Some(Vec2::new(113.0, 154.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0., 0., 0.),
                        ..default()
                    },
                    2 => SpriteBundle {
                        texture: asset_server.load(
                            "./Hectares/Cards/Recurso - Guano.png"),
                        sprite: Sprite{
                        
                            custom_size: Some(Vec2::new(113.0, 154.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0., 0., 0.),
                        ..default()
                    },
                    _ => SpriteBundle {
                        texture: asset_server.load(
                            "./Hectares/Cards/Recurso - Guano.png"),
                        sprite: Sprite{
                        
                            custom_size: Some(Vec2::new(113.0, 154.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0., 0., 0.),
                        ..default()
                    },
                },

                PickableBundle::default(), // <- Makes the mesh pickable.
                On::<Pointer<DragStart>>::target_insert(Dragging), // <- Add the Dragging component to the entity    
                On::<Pointer<DragEnd>>::target_remove::<Dragging>(), // <- Remove the Dragging component from the entity
                On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                    transform.translation.x += drag.delta.x; // Make the square follow the mouse
                    transform.translation.y -= drag.delta.y;

                }),

                match coin {
                    0 => On::<Pointer<Up>>::run(Plantar),
                    1 => On::<Pointer<Up>>::run(Regar),
                    2 => On::<Pointer<Up>>::run(Fertilizar),
                    _ => On::<Pointer<Up>>::run(Plantar),
                },
                

                ));
    }

}



//TODO: mudar o nome pois a ideia é que isso é disparado quando a planta é colhida
//TODO: fazer a planta ser colhida e virar uma carta
fn Colhe (
    mut events: EventReader<ColheuPlanta>,
    mut ouro: ResMut<Gold>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut inventario: ResMut<Inventario>,

)
{
    for event in events.read() {
        //info!("Colheu a planta {:#?}", event.0);    
        let tipodeplanta = &event.0;

        for item in inventario.itens.iter_mut() {
            if item.tipo == *tipodeplanta {
                item.quantidade += 1;
                //item.visivel = tr;
                return;
            }
        }
    }
/*
       // ouro.0 += 10;
        let mut rng = thread_rng();
        let x = rng.gen_range(-100.0..100.0);
            
            let coin = rng.gen_range(0 .. 2);

            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_secs(1),
                TransformPositionLens{
                    start:Vec3::new(0., 0., LAYER_3_1),
                    end:Vec3::new(x, -350. + x, LAYER_3_1),
                }
            );

            let texturacarta = match tipodeplanta {
                TipoPlantas::CouveRosa => "Hectares/Cards/Plantas/Couve Flor/Carta.png",
                TipoPlantas::Hedrazeba => "Hectares/Cards/Plantas/Hedrazeba/Carta.png",
                TipoPlantas::GargomiloMiudo => "Hectares/Cards/Plantas/Gargomilo Miudo/Carta.png",
                TipoPlantas::FolhaGorda => "Hectares/Cards/Plantas/Folha Gorda/Carta.png",
                TipoPlantas::CenouraPimenta => "Hectares/Cards/Plantas/Cenoura Pimenta/Carta.png",
                TipoPlantas::FlorAmarela => "Hectares/Cards/Plantas/Flor Amarela/Carta.png",
            };
        //TODO: USAR EVENTO DE SPAWNA CARTA AQUI AO INVES DE DUPLICAR !!! PROCURAR POR OUTROS LUGARES QUE ESTAO DUPLICANDO 
            commands.spawn((

                TipoPlanta(tipodeplanta.clone()), //isso vai entrar nas cartas erradas tb assim !
                TipoCarta(TiposCarta::Semente),           
                Animator::new(tween),                                               
                    
                    SpriteBundle {
                        texture: asset_server.load(texturacarta),
                        sprite: Sprite{
                        
                            custom_size: Some(Vec2::new(113.0, 154.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.,0.,LAYER_3_1),
                        ..default()
                    },
                    
                    PickableBundle::default(), // <- Makes the mesh pickable.
                    On::<Pointer<DragStart>>::target_insert(Dragging), // <- Add the Dragging component to the entity    
                    On::<Pointer<DragEnd>>::target_remove::<Dragging>(), // <- Remove the Dragging component from the entity
                    On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                    transform.translation.x += drag.delta.x; // Make the square follow the mouse
                    transform.translation.y -= drag.delta.y;

                }),
                    On::<Pointer<Up>>::run(Plantar),
                ));
            

            }
            */
}


// --------------------------------------------------SETUP-----------------------------
fn Setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut windows: Query  <&mut Window>,
    mut eventosSemente: EventWriter<eSpawnaSemente>,
    mut eventosMensagem: EventWriter<eAbreMensagem>,
    eventosCos: EventWriter<eSpawnaCosmetico>,
    eventos: EventWriter<eSpawnaCarta>,
    //mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    let mut window = windows.single_mut();

    let mut my_2d_camera_bundle = Camera2dBundle::default();
    //my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1600.);
    
    my_2d_camera_bundle.transform.scale = vec3(1.2,1.2,1.2);

    commands.spawn(
        (
    my_2d_camera_bundle,

        
    )

    );

// SETUP UI



commands.spawn (
    SpriteBundle {
        texture: asset_server.load("./Hectares/Cards/Fundo.png"),
        sprite: Sprite{
        
            custom_size: Some(Vec2::new(503.0, 908.0)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0.0, LAYER_0),
        ..default()
    }
);

commands.spawn(TextBundle {
    text: Text::from_section(
    "            HECTARES 0.1 - by ZeD",
    TextStyle {
    color: Color::WHITE,
    font: asset_server.load("./Font/PressStart2P.ttf"),
    ..default()
    },
    ),
    transform: Transform::from_translation(Vec3::new(10.0, 15.0, LAYER_2)),
    ..Default::default()
});
commands.spawn(
    
   (
    Dia(1),
     Text2dBundle {
    
    text: Text::from_section(
    "1",
    TextStyle {
    color: Color::WHITE,
    font: asset_server.load("./Font/PressStart2P.ttf"),
    font_size: 20.0,
    ..default()
    },
    ),
    transform: Transform::from_translation(Vec3::new(30.0, 400.0, LAYER_2)),
    ..Default::default()
}));

let dindin = 0; 
commands.spawn((
    UIDinheiro(dindin),
    Text2dBundle {
        transform: Transform::from_translation(Vec3::new(-135., 390.0, LAYER_2)),
        text: Text::from_section(
            dindin.to_string(),
            TextStyle {
                color: Color::BLACK,
                font: asset_server.load("./Font/PressStart2P.ttf"),
                font_size: 30.0,
                ..default()
            },
        ),
        
        ..Default::default()
    })
);


commands.spawn (
   ( 
    OverlayCor,
    SpriteBundle {

        texture: asset_server.load("./Hectares/Cards/Overlaycor.png"),
        sprite: Sprite{
            color: Color::rgba(0.0, 0.0, 0.0, 0.0), //cor boa para dia
            //color: Color::rgba(0.2, 0.3, 0.9, 1.0), //cor boa para noite
            //color: Color::rgba(0.2, 0.4, 0.4, 1.0), //cor boa ewsverdeada
            //color: Color::rgba(0.9, 0.5, 0.4, 0.8), //cor boa avermelhada laranja
            //color: Color::rgba(0.9, 0.9, 0.4, 0.8), //cor boa amarela
            //color: Color::rgba(0.4, 0.9, 0.4, 0.8), //cor boa verde
            //color: Color::rgba(0.4, 0.9, 0.9, 0.8), //cor boa azul
            //color: Color::rgba(0.9, 0.4, 0.9, 0.8), //cor boa rosa

            custom_size: Some(Vec2::new(503.0, 908.0)), 
            ..default()
        },
        transform: Transform::from_xyz(0., 0.0, LAYER_5),
        ..default()
    },
    Pickable::IGNORE,


)
);

// SOMBRA DA MAO
commands.spawn (
    (SpriteBundle {
        texture: asset_server.load("./Hectares/Cards/Sombra Mao.png"),
        sprite: Sprite{     
            color: Color::rgb(0.3, 0.1, 0.1),     
            custom_size: Some(Vec2::new(638.0, 416.0)),
            ..default()
        },
        transform: Transform::from_xyz(0., -300.0, LAYER_1),
        ..default()
    }, Pickable::IGNORE,)
);

commands.spawn ((
    SpriteBundle {
        texture: asset_server.load("./Hectares/Cards/top-agua-cheia.png"),
        sprite: Sprite{     
            custom_size: Some(Vec2::new(84.0 /1.2, 93.0/1.2)),
            ..default()
        },
        transform: Transform::from_xyz(100., 390.0, LAYER_1),
        ..default()
    },
    PickableBundle::default(),
    On::<Pointer<Click>>::run(SpawnaAgua),
)
);

commands.spawn ((
    Inseto(TipoInseto::Mosca),
    SpriteBundle {
        texture: asset_server.load("./Hectares/Cards/Mosca beliscadora.png"),
        sprite: Sprite{     
            custom_size: Some(Vec2::new(81.0 /2.0, 85.0/2.0)),
            ..default()
        },
        transform: Transform::from_xyz(400., 110.0, LAYER_3),
        ..default()
    },
    Pickable::IGNORE,
  //  On::<Pointer<Click>>::run(SpawnaAgua),
)
);



commands.spawn ((
    Relogio(FaseDia::MeioManha),
    SpriteBundle {
        texture: asset_server.load("./Hectares/Cards/Meio da manha.png"),
        sprite: Sprite{     
            custom_size: Some(Vec2::new(83.0 /1.2, 92.0/1.2)),
            ..default()
        },
        transform: Transform::from_xyz(-20., 390.0, LAYER_1),
        ..default()
    },
    Pickable::IGNORE,
  //  On::<Pointer<Click>>::run(SpawnaAgua),
)
);

commands.spawn (
    (SpriteBundle {
        texture: asset_server.load("./Hectares/Cards/top-settings.png"),
        sprite: Sprite{     
            custom_size: Some(Vec2::new(84.0 / 1.2, 93.0 / 1.2)),
            ..default()
        },
        transform: Transform::from_xyz(84.0 * 2.2, 390.0, LAYER_1),
        ..default()
    },
    PickableBundle::default(),
    On::<Pointer<Click>>::run(AbreTransmutador),
)
);
commands.spawn ((
    UIMercado,
    SpriteBundle {
        texture: asset_server.load("./Hectares/Cards/top-dinheiro.png"),
        sprite: Sprite{     
            custom_size: Some(Vec2::new(241.0 /1.5, 93.0/1.5 )),
            ..default()
        },
        transform: Transform::from_xyz(-160.0, 390.0, LAYER_1),
        ..default()
    },
    PickableBundle::default(),
    On::<Pointer<Click>>::run(AbreMercado),

    //On::<Pointer<Drop>>::send_event::<eVender>(), // <- Send a plantar event when the entity is dropped
  //  On::<Pointer<Up>>::run(Vender),
));
// SETUP SLOTS
for y in -2..6 {
    for x in -2..2 {

        let tempid = commands.spawn ((
            Slot,
            Selecionado(false),
            SpriteBundle {
                texture: asset_server.load("./Hectares/Cards/Crop OL.png"),
                sprite: Sprite{
                
                    custom_size: Some(Vec2::new(80., 87.0)),
                    ..default()
                },
         
                transform: Transform::from_xyz((x as f32  * 85.) + 50., (y as f32 * 85.) - 100. , LAYER_0),
             
                ..default()
            },
            //PbrBundle::default(), 
               PickableBundle::default(),
             //  On::<Pointer<Drop>>::send_event::<plantar>(), // <- Send a plantar event when the entity is dropped

        )).id();

        if y == 0 && x == 0 {

            let temp = commands.spawn ((
                Ninho,
                Selecionado(false),
                SpriteBundle {
                    texture: asset_server.load("./Hectares/Cards/insetos/Hive.png"),
                    sprite: Sprite{
                    
                        custom_size: Some(Vec2::new(80., 100.0)),
                        ..default()
                    },
             
                    transform: Transform::from_xyz((x as f32  * 85.) + 50., (y as f32 * 85.) - 100. , LAYER_2),
                 
                    ..default()
                },
                //PbrBundle::default(), 
                   PickableBundle::default(),
                   On::<Pointer<Click>>::run(ColheMel),
                 //  On::<Pointer<Drop>>::send_event::<plantar>(), // <- Send a plantar event when the entity is dropped
    
            )).id();
            

            commands.spawn ((
                Abelha,
                //Inseto(TipoInseto::Mosca),
                Target(temp),

                EstadoAbelhaAtual(EstadoAbelha::IndoNinho),
                SpriteBundle {
                    texture: asset_server.load("./Hectares/Cards/insetos/MQQ.png"),
                    sprite: Sprite{     
                        custom_size: Some(Vec2::new(81.0 /2.0, 85.0/2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(400., 110.0, LAYER_3),
                    ..default()
                },
                Pickable::IGNORE,
                //On::<Pointer<Click>>::run(SpawnaAgua),
            )
            );
            commands.entity(tempid).insert(Semeado);
       
                
        }


    }
}
//window.resolution.set(508., 903.); //faz um zoomout pra caber tudo na camera!!!!!!!!!!!!!!!!!!!!
window.resolution.set(412., 760.);
window.present_mode = PresentMode::AutoNoVsync;
window.resizable = false;


window.title = "Hectares".to_string();




//eventosMensagem.send(eAbreMensagem());
    


   // eventosMensagem.send(eAbreMensagem());

// DÁ MÃO INICIAL , ISSO SERA SUBSTITUIDO
    

    

    
    


}



//Implementar sistema de particulas para regar a planta
//implementar sistema que faz as plantas ficarem animadas



#[derive(Event)]
struct Explosion(Vec2);


#[derive(Component)]
pub struct Particle {
    position: Vec2,
    velocity: f32,
    size: f32,
    color: Color,
    direction: Vec2,
    time: f32,
    lapsed: f32,
}

fn particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut events: EventReader<Explosion>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    
) {
   
    for event in events.read() {
        let mut rng = thread_rng();
        let position = event.0;
        
        for _ in 0..18 {
    
        commands.spawn((
            Particle {
                position,
                velocity: rng.gen_range(50.0..150.0),
                size: 5.0,
                color: Color::rgb(1.0, 0.0, 0.0),
                direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                time: time.elapsed_seconds(), 
                lapsed: 0.0, 
            },
            
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: rng.gen_range(15.5..20.0) })),
                material: materials.add(Color::hsl(rng.gen_range(71.5..98.0), 1.0, 1.0)),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT to +X_EXTENT.
                    position.x + rng.gen_range(-10.0..10.0),
                    position.y + rng.gen_range(-10.0..10.0),
                    LAYER_6,
                ),
                ..default()
            },        
        ));

    }
        


    }
}

fn move_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Mesh2dHandle )>,
) {

    for (entity, mut particle, mut transform,mesh) in &mut query {
        let direcao = particle.direction.clone();
        let velocidade = particle.velocity.clone();

        particle.lapsed += time.delta_seconds();
        
        particle.position += direcao * velocidade * time.delta_seconds();
        transform.translation = particle.position.extend(0.0);
        transform.translation.z = LAYER_6;
        //transform.scale -= 0.025 * velocidade *  time.delta_seconds();


        if particle.lapsed > 0.5 {
            commands.entity(entity).despawn();
        }

    }
    

}


//---------------------------------fim particulas -------------------------
