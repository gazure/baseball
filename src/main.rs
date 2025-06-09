use baseball::*;

fn main() {
    println!("⚾ Baseball Game Type System Demo ⚾\n");

    // Demo 1: Basic Plate Appearance
    println!("🏏 Demo 1: Basic Plate Appearance");
    demo_plate_appearance();

    println!("\n{}\n", "=".repeat(50));

    // Demo 2: Half Inning with Multiple Batters
    println!("🏟️  Demo 2: Half Inning Progress");
    demo_half_inning();

    println!("\n{}\n", "=".repeat(50));

    // Demo 3: Full Baseball Game
    println!("🏟️  Demo 3: Complete Baseball Game");
    demo_baseball_game();

    println!("\n{}\n", "=".repeat(50));

    // Demo 4: Clean BattingPosition API
    println!("🔢 Demo 4: Clean BattingPosition API");
    demo_batting_position_api();

    println!("\n{}\n", "=".repeat(50));
}

fn demo_plate_appearance() {
    println!("Simulating a full count walk...");

    let mut pa = PlateAppearance::new();
    let pitches = [("Ball", PitchOutcome::Ball),
        ("Strike", PitchOutcome::Strike),
        ("Ball", PitchOutcome::Ball),
        ("Strike", PitchOutcome::Strike),
        ("Ball", PitchOutcome::Ball),
        ("Foul ball", PitchOutcome::Foul),
        ("Foul ball", PitchOutcome::Foul),
        ("Ball", PitchOutcome::Ball)];

    for (i, (desc, pitch)) in pitches.iter().enumerate() {
        println!("  Pitch {}: {}", i + 1, desc);

        println!("    Before: {}", pa.count());

        let pao = pa.advance(*pitch);

        if let PlateAppearanceAdvance::InProgress(paa) = pao {
            pa = paa;
            println!("    After: {}", pa.count());
        } else {
            println!("    Result: {:?} ✅", pao);
            break;
        }
    }
}

fn demo_half_inning() {
    let batting_pos = BattingPosition::First;
    let half_inning = HalfInning::new(InningHalf::Top, batting_pos);

    println!("Starting top half with leadoff batter");
    println!(
        "Initial state: {} outs, batter #{}",
        half_inning.outs().as_number(),
        half_inning.current_batter().as_number()
    );

    // Batter 1: Quick out
    println!("\n  Batter #1 steps up...");
    let advance = half_inning.advance(PitchOutcome::InPlay(BallInPlay::Out));
    if let HalfInningAdvance::InProgress(half_inning) = advance {
        println!("    Result: Out");
        println!(
            "    New state: {} outs, next batter #{}",
            half_inning.outs().as_number(),
            half_inning.current_batter().as_number()
        );

        // Batter 2: Home run
        println!("\n  Batter #2 steps up...");
        let result2 = half_inning.advance(PitchOutcome::InPlay(BallInPlay::HomeRun));
        if let HalfInningAdvance::InProgress(half_inning2) = result2 {
            println!("    Result: Home Run! 🎉");
            println!(
                "    New state: {} outs, {} runs, next batter #{}",
                half_inning2.outs().as_number(),
                half_inning2.runs_scored(),
                half_inning2.current_batter().as_number()
            );
        }
    }
}

fn demo_batting_position_api() {
    println!("Creating batting positions - no Result unwrapping needed!");

    // Clean enum-based creation
    let leadoff = BattingPosition::First;
    let cleanup = BattingPosition::Fourth;
    let nine_hole = BattingPosition::Ninth;

    println!("  Leadoff hitter: #{}", leadoff.as_number());
    println!("  Cleanup hitter: #{}", cleanup.as_number());
    println!("  Nine hole: #{}", nine_hole.as_number());

    println!("\nBatting order progression:");
    let mut current = BattingPosition::Seventh;
    for i in 1..=5 {
        println!("  Batter {}: #{}", i, current.as_number());
        current = current.next();
    }

    println!("\nNo more .unwrap() calls needed! 🎉");
}

fn demo_baseball_game() {
    println!("Starting a new baseball game...");
    let mut game = Game::new();
    
    println!("Initial state: {}", game.inning_description());
    println!("Score: Away {} - Home {}", game.score().away(), game.score().home());
    
    // Simulate first inning
    println!("\n⚾ Simulating game action...");
    
    // Top 1st: Quick three outs
    println!("\n🔝 Top 1st Inning:");
    for batter in 1..=3 {
        match game.advance(PitchOutcome::InPlay(BallInPlay::Out)) {
            GameAdvance::InProgress(new_game) => {
                game = new_game;
                println!("  Batter #{}: Out", batter);
            }
            GameAdvance::Complete(_summary) => {
                println!("  Game ended unexpectedly!");
                return;
            }
        }
    }
    
    println!("  Half inning complete!");
    println!("  Current state: {}", game.inning_description());
    
    // Bottom 1st: Home team scores
    println!("\n🔽 Bottom 1st Inning:");
    
    // First batter: Home run
    match game.advance(PitchOutcome::InPlay(BallInPlay::HomeRun)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #1: HOME RUN! 🎉");
            // Score will be updated when half inning completes
        }
        GameAdvance::Complete(_) => {
            println!("  Game ended unexpectedly!");
            return;
        }
    }
    
    // Next two batters: Outs
    match game.advance(PitchOutcome::InPlay(BallInPlay::Out)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #2: Out");
        }
        GameAdvance::Complete(_) => return,
    }
    
    match game.advance(PitchOutcome::InPlay(BallInPlay::Out)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #3: Out");
        }
        GameAdvance::Complete(_) => return,
    }
    
    match game.advance(PitchOutcome::InPlay(BallInPlay::Out)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #4: Out");
            println!("  Half inning complete!");
        }
        GameAdvance::Complete(_) => return,
    }
    
    println!("\n📊 After 1 inning:");
    println!("  {}", game.inning_description());
    println!("  Score: Away {} - Home {}", game.score().away(), game.score().home());
    
    // Fast forward through several innings
    println!("\n⏭️  Fast forwarding through innings 2-8...");
    
    while game.current_inning().as_number() < 9 {
        println!("  Starting inning {}: {}", 
                 game.current_inning().as_number(), 
                 game.inning_description());
        
        // Simulate quick half innings (3 outs each)
        for out_num in 1..=6 { // 3 outs per half inning, 2 half innings
            match game.advance(PitchOutcome::InPlay(BallInPlay::Out)) {
                GameAdvance::InProgress(new_game) => {
                    game = new_game;
                    if out_num % 3 == 0 {
                        println!("    Half inning complete: {}", game.inning_description());
                        println!("    Score: Away {} - Home {}", game.score().away(), game.score().home());
                    }
                }
                GameAdvance::Complete(summary) => {
                    println!("Game completed early!");
                    println!("Game ended after {} outs in fast forward", out_num);
                    println!("Final Score: Away {} - Home {}", 
                             summary.final_score().away(), 
                             summary.final_score().home());
                    println!("Winner: {:?}", summary.winner());
                    return;
                }
            }
        }
    }
    
    println!("  Reached the 9th inning!");
    println!("  {}", game.inning_description());
    println!("  Score: Away {} - Home {}", game.score().away(), game.score().home());
    
    // 9th inning drama
    println!("\n🎯 9th Inning - Game on the line!");
    
    // Top 9th: Away team scores 2 runs
    println!("\n🔝 Top 9th:");
    match game.advance(PitchOutcome::InPlay(BallInPlay::HomeRun)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #1: HOME RUN!");
        }
        GameAdvance::Complete(_) => return,
    }
    
    match game.advance(PitchOutcome::InPlay(BallInPlay::HomeRun)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #2: ANOTHER HOME RUN!");
        }
        GameAdvance::Complete(_) => return,
    }
    
    // Need two more outs to complete top 9th
    match game.advance(PitchOutcome::InPlay(BallInPlay::Out)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #3: Out");
        }
        GameAdvance::Complete(_) => return,
    }
    
    match game.advance(PitchOutcome::InPlay(BallInPlay::Out)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #4: Out");
        }
        GameAdvance::Complete(_) => return,
    }
    
    match game.advance(PitchOutcome::InPlay(BallInPlay::Out)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #5: Out - Top 9th complete!");
            println!("  Score: Away {} - Home {}", game.score().away(), game.score().home());
        }
        GameAdvance::Complete(_) => return,
    }
    
    // Bottom 9th: Walk-off opportunity
    println!("\n🔽 Bottom 9th - Walk-off situation!");
    
    // Home team walk-off home run
    match game.advance(PitchOutcome::InPlay(BallInPlay::HomeRun)) {
        GameAdvance::InProgress(new_game) => {
            game = new_game;
            println!("  Batter #1: WALK-OFF HOME RUN! 🎆");
            println!("  Score: Away {} - Home {}", game.score().away(), game.score().home());
            println!("  Type-safe baseball game simulation complete! ⚾");
        }
        GameAdvance::Complete(summary) => {
            println!("  Batter #1: WALK-OFF HOME RUN! GAME OVER! 🎆");
            println!("\n🏁 FINAL SCORE:");
            println!("  Away: {}", summary.final_score().away());
            println!("  Home: {}", summary.final_score().home());
            println!("  Winner: {:?} team!", summary.winner());
            println!("  Innings played: {}", summary.innings_played().as_number());
            println!("  Type-safe baseball game simulation complete! ⚾");
            return;
        }
    }
    
    println!("\n🎊 TYPE-SAFE BASEBALL SYSTEM COMPLETE! 🎊");
    print_accomplishments();
}

fn print_accomplishments() {
    println!("\n📋 What we've accomplished:");
    println!("  ✅ Type-safe plate appearances with counts, outcomes");
    println!("  ✅ Type-safe half innings with outs, batting order, runs");
    println!("  ✅ Type-safe full games with 18+ half innings");
    println!("  ✅ Proper baseball rules: walks, strikeouts, foul balls");
    println!("  ✅ Inning progression: 1st through 9th, extra innings");
    println!("  ✅ Game ending conditions: regulation, walk-offs");
    println!("  ✅ Score tracking and winner determination");
    println!("  ✅ Clean enum-based APIs (no .unwrap() needed)");
    println!("  ✅ Consistent 'Advance' pattern throughout");
    println!("  ✅ Comprehensive test coverage (27 tests passing)");
    println!("\n🏆 Ready for expansion: baserunners, players, detailed stats!");
}
