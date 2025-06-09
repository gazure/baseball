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

    // Demo 3: Clean BattingPosition API
    println!("🔢 Demo 3: Clean BattingPosition API");
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
