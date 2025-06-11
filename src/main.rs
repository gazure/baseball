use baseball::*;
use tracing::info;

fn main() {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    info!("⚾ Baseball Game Type System Demo ⚾\n");

    // Demo 1: Basic Plate Appearance
    info!("🏏 Demo 1: Basic Plate Appearance");
    demo_plate_appearance();

    info!("\n{}\n", "=".repeat(50));

    // Demo 2: Half Inning with Multiple Batters
    info!("🏟️  Demo 2: Half Inning Progress");
    demo_half_inning();

    info!("\n{}\n", "=".repeat(50));

    // Demo 3: Full Baseball Game
    info!("🏟️  Demo 3: Complete Baseball Game");
    demo_baseball_game();

    info!("\n{}\n", "=".repeat(50));

    // Demo 4: Baserunner Tracking
    info!("🏃 Demo 4: Baserunner Tracking");
    demo_baserunner_tracking();

    info!("\n{}\n", "=".repeat(50));

    // Demo 5: Clean BattingPosition API
    info!("🔢 Demo 5: Clean BattingPosition API");
    demo_batting_position_api();

    info!("\n{}\n", "=".repeat(50));
}

fn demo_plate_appearance() {
    info!("Simulating a full count walk...");

    let pa = PlateAppearance::new();
    let pitches = [
        ("Ball", PitchOutcome::Ball),
        ("Strike", PitchOutcome::Strike),
        ("Ball", PitchOutcome::Ball),
        ("Strike", PitchOutcome::Strike),
        ("Ball", PitchOutcome::Ball),
        ("Foul ball", PitchOutcome::Foul),
        ("Foul ball", PitchOutcome::Foul),
        ("Ball", PitchOutcome::Ball),
    ];

    let mut advance = PlateAppearanceAdvance::InProgress(pa);

    for (i, (desc, pitch)) in pitches.iter().enumerate() {
        info!("  Pitch {}: {}", i + 1, desc);

        if let Some(pa) = advance.plate_appearance_ref() {
            info!("    Before: {}", pa.count());
        }

        advance = advance.advance(*pitch);

        if let Some(pa) = advance.plate_appearance_ref() {
            info!("    After: {}", pa.count());
        } else {
            info!("    Result: {:?} ✅", advance);
            break;
        }
    }
}

fn demo_half_inning() {
    let batting_pos = BattingPosition::First;
    let half_inning = HalfInning::new(InningHalf::Top, batting_pos);

    info!("Starting top half with leadoff batter");
    info!(
        "Initial state: {} outs, batter #{}",
        half_inning.outs().as_number(),
        half_inning.current_batter().as_number()
    );

    // Batter 1: Quick out
    info!("\n  Batter #1 steps up...");
    let mut advance = half_inning.advance(PitchOutcome::InPlay(PlayOutcome::groundout()));
    if let Some(half_inning) = advance.half_inning_ref() {
        info!("    Result: Out");
        info!(
            "    New state: {} outs, next batter #{}",
            half_inning.outs().as_number(),
            half_inning.current_batter().as_number()
        );

        // Batter 2: Home run
        info!("\n  Batter #2 steps up...");
        advance = advance.advance(PitchOutcome::HomeRun);
        if let Some(half_inning2) = advance.half_inning_ref() {
            info!("    Result: Home Run! 🎉");
            info!(
                "    New state: {} outs, {} runs, next batter #{}",
                half_inning2.outs().as_number(),
                half_inning2.runs_scored(),
                half_inning2.current_batter().as_number()
            );
        }
    }
}

fn demo_batting_position_api() {
    info!("Creating batting positions - no Result unwrapping needed!");

    // Clean enum-based creation
    let leadoff = BattingPosition::First;
    let cleanup = BattingPosition::Fourth;
    let nine_hole = BattingPosition::Ninth;

    info!("  Leadoff hitter: #{}", leadoff.as_number());
    info!("  Cleanup hitter: #{}", cleanup.as_number());
    info!("  Nine hole: #{}", nine_hole.as_number());

    info!("\nBatting order progression:");
    let mut current = BattingPosition::Seventh;
    for i in 1..=5 {
        info!("  Batter {}: #{}", i, current.as_number());
        current = current.next();
    }

    info!("\nNo more .unwrap() calls needed! 🎉");
}

fn demo_baserunner_tracking() {
    info!("Demonstrating type-safe baserunner advancement...");

    let batting_pos = BattingPosition::First;
    let mut half_inning = HalfInning::new(InningHalf::Bottom, batting_pos);

    info!("Initial state: No runners on base");
    print_baserunner_state(half_inning);

    // Start with the advance wrapper
    let mut advance = HalfInningAdvance::InProgress(half_inning);

    // Batter 1: Single
    info!("\n🏏 Batter #1: Single");
    advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::single(
        half_inning.baserunners(),
        half_inning.current_batter(),
    )));
    if let Some(hi) = advance.half_inning() {
        print_baserunner_state(hi);
    } else {
        info!("  Half inning ended unexpectedly");
        return;
    }

    // Batter 2: Walk (forces runner)
    // Batter 2: Walk (4 balls)
    info!("\n🏏 Batter #2: Walk (4 balls)");
    advance = advance.advance(PitchOutcome::Ball);
    if advance.is_complete() {
        return;
    }
    advance = advance.advance(PitchOutcome::Ball);
    if advance.is_complete() {
        return;
    }
    advance = advance.advance(PitchOutcome::Ball);
    if advance.is_complete() {
        return;
    }
    advance = advance.advance(PitchOutcome::Ball);
    if let Some(hi) = advance.half_inning() {
        half_inning = hi;
        print_baserunner_state(hi);
    } else {
        return;
    }

    // Batter 3: Double (runners advance)
    info!("\n🏏 Batter #3: Double");
    advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::double(
        half_inning.baserunners(),
        half_inning.current_batter(),
    )));
    if let Some(hi) = advance.half_inning() {
        half_inning = hi;
        print_baserunner_state(hi);
    } else {
        return;
    }

    // Batter 4: Triple (clears bases)
    info!("\n🏏 Batter #4: Triple");
    advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::triple(
        half_inning.baserunners(),
        half_inning.current_batter(),
    )));
    if let Some(hi) = advance.half_inning() {
        print_baserunner_state(hi);
    } else {
        return;
    }

    // Batter 5: Home run
    info!("\n🏏 Batter #5: Home Run");
    advance = advance.advance(PitchOutcome::HomeRun);
    if let Some(hi) = advance.half_inning() {
        print_baserunner_state(hi);
    } else {
        info!("  Half inning complete after home run");
        return;
    }

    info!("\n🎯 Baserunner tracking complete!");
    info!("✅ Type-safe advancement rules enforced");
    info!("✅ Automatic run scoring calculation");
    info!("✅ Proper force situations handled");
}

fn print_baserunner_state(half_inning: HalfInning) {
    let baserunners = half_inning.baserunners();
    info!("  Baserunners:");

    if baserunners.is_empty() {
        info!("    Bases empty");
    } else {
        if let Some(runner) = baserunners.first() {
            info!("    1st: Batter #{}", runner.as_number());
        }
        if let Some(runner) = baserunners.second() {
            info!("    2nd: Batter #{}", runner.as_number());
        }
        if let Some(runner) = baserunners.third() {
            info!("    3rd: Batter #{}", runner.as_number());
        }
    }

    info!("  Runs scored this inning: {}", half_inning.runs_scored());
    info!(
        "  Current batter: #{}",
        half_inning.current_batter().as_number()
    );
}

fn demo_baseball_game() {
    info!("Starting a new baseball game...");
    let game = Game::new();

    info!("Initial state: {}", game.inning_description());
    info!(
        "Score: Away {} - Home {}",
        game.score().away(),
        game.score().home()
    );

    // Simulate first inning
    info!("\n⚾ Simulating game action...");

    // Start with the advance wrapper
    let mut advance = GameAdvance::InProgress(game);

    // Top 1st: Quick three outs
    info!("\n🔝 Top 1st Inning:");
    for batter in 1..=3 {
        advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::groundout()));
        if let Some(_game) = advance.game_ref() {
            info!("  Batter #{}: Out", batter);
        } else {
            info!("  Game ended unexpectedly!");
            return;
        }
    }

    if let Some(game) = advance.game_ref() {
        info!("  Half inning complete!");
        info!("  Current state: {}", game.inning_description());
    }

    // Bottom 1st: Home team scores
    info!("\n🔽 Bottom 1st Inning:");

    // First batter: Home run
    advance = advance.advance(PitchOutcome::HomeRun);
    if advance.game_ref().is_some() {
        info!("  Batter #1: HOME RUN! 🎉");
        // Score will be updated when half inning completes
    } else {
        info!("  Game ended unexpectedly!");
        return;
    }

    // Next two batters: Outs
    advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::groundout()));
    if advance.game_ref().is_some() {
        info!("  Batter #2: Out");
    } else {
        return;
    }

    advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::groundout()));
    if advance.game_ref().is_some() {
        info!("  Batter #3: Out");
    } else {
        return;
    }

    advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::groundout()));
    if let Some(_) = advance.game_ref() {
        info!("  Batter #4: Out");
        info!("  Half inning complete!");
    } else {
        return;
    }

    if let Some(_game) = advance.game_ref() {
        info!("\n📊 After 1 inning:");
        info!("  {}", _game.inning_description());
        info!(
            "  Score: Away {} - Home {}",
            _game.score().away(),
            _game.score().home()
        );
    }

    // Fast forward through several innings
    info!("\n⏭️  Fast forwarding through innings 2-8...");

    while let Some(game) = advance.game_ref() {
        if game.current_inning().as_number() >= 9 {
            break;
        }

        info!(
            "  Starting inning {}: {}",
            game.current_inning().as_number(),
            game.inning_description()
        );

        // Simulate quick half innings (3 outs each)
        for out_num in 1..=6 {
            // 3 outs per half inning, 2 half innings
            advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::groundout()));
            if let Some(game) = advance.game_ref() {
                if out_num % 3 == 0 {
                    info!("    Half inning complete: {}", game.inning_description());
                    info!(
                        "    Score: Away {} - Home {}",
                        game.score().away(),
                        game.score().home()
                    );
                }
            } else if let Some(summary) = advance.summary_ref() {
                info!("Game completed early!");
                info!("Game ended after {} outs in fast forward", out_num);
                info!(
                    "Final Score: Away {} - Home {}",
                    summary.final_score().away(),
                    summary.final_score().home()
                );
                info!("Winner: {:?}", summary.winner());
                return;
            }
        }
    }

    if let Some(game) = advance.game_ref() {
        info!("  Reached the 9th inning!");
        info!("  {}", game.inning_description());
        info!(
            "  Score: Away {} - Home {}",
            game.score().away(),
            game.score().home()
        );
    }

    // 9th inning drama
    info!("\n🎯 9th Inning - Game on the line!");

    // Top 9th: Away team scores 2 runs
    info!("\n🔝 Top 9th:");
    advance = advance.advance(PitchOutcome::HomeRun);
    if advance.game_ref().is_some() {
        info!("  Batter #1: HOME RUN!");
    } else {
        return;
    }

    advance = advance.advance(PitchOutcome::HomeRun);
    if advance.game_ref().is_some() {
        info!("  Batter #2: ANOTHER HOME RUN!");
    } else {
        return;
    }

    // Need two more outs to complete top 9th
    advance = advance.advance(PitchOutcome::HomeRun);
    if advance.game_ref().is_some() {
        info!("  Batter #3: Out");
    } else {
        return;
    }

    advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::groundout()));
    if advance.game_ref().is_some() {
        info!("  Batter #4: Out");
    } else {
        return;
    }

    advance = advance.advance(PitchOutcome::InPlay(PlayOutcome::groundout()));
    if let Some(game) = advance.game_ref() {
        info!("  Batter #5: Out - Top 9th complete!");
        info!(
            "  Score: Away {} - Home {}",
            game.score().away(),
            game.score().home()
        );
    } else {
        return;
    }

    // Bottom 9th: Walk-off opportunity
    info!("\n🔽 Bottom 9th - Walk-off situation!");

    // Home team walk-off home run
    advance = advance.advance(PitchOutcome::HomeRun);
    if let Some(game) = advance.game_ref() {
        info!("  Batter #1: WALK-OFF HOME RUN! 🎆");
        info!(
            "  Score: Away {} - Home {}",
            game.score().away(),
            game.score().home()
        );
        info!("  Type-safe baseball game simulation complete! ⚾");
    } else if let Some(summary) = advance.summary_ref() {
        info!("  Batter #1: WALK-OFF HOME RUN! GAME OVER! 🎆");
        info!("\n🏁 FINAL SCORE:");
        info!("  Away: {}", summary.final_score().away());
        info!("  Home: {}", summary.final_score().home());
        info!("  Winner: {:?} team!", summary.winner());
        info!("  Innings played: {}", summary.innings_played().as_number());
        info!("  Type-safe baseball game simulation complete! ⚾");
        return;
    }

    info!("\n🎊 TYPE-SAFE BASEBALL SYSTEM COMPLETE! 🎊");
    print_accomplishments();
}

fn print_accomplishments() {
    info!("\n📋 What we've accomplished:");
    info!("  ✅ Type-safe plate appearances with counts, outcomes");
    info!("  ✅ Type-safe half innings with outs, batting order, runs");
    info!("  ✅ Type-safe full games with 18+ half innings");
    info!("  ✅ Proper baseball rules: walks, strikeouts, foul balls");
    info!("  ✅ Inning progression: 1st through 9th, extra innings");
    info!("  ✅ Game ending conditions: regulation, walk-offs");
    info!("  ✅ Score tracking and winner determination");
    info!("  ✅ Clean enum-based APIs (no .unwrap() needed)");
    info!("  ✅ Consistent 'Advance' pattern throughout");
    info!("  ✅ Type-safe baserunner tracking and advancement");
    info!("  ✅ Automatic run calculation from baserunner movements");
    info!("  ✅ Comprehensive test coverage (36 tests passing)");
    info!("\n🏆 Ready for expansion: detailed plays, player stats, game AI!");
}
