use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::{RichText, Color32};


struct Images {
    sugarfunge_logo: Handle<Image>,
    avatar: Handle<Image>,
    mail: Handle<Image>,
    bell: Handle<Image>,
    cogwheel: Handle<Image>
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            sugarfunge_logo: asset_server.load("sugarfunge_logo.png"),
            avatar: asset_server.load("avatar.png"),
            mail: asset_server.load("mail.png"),
            bell: asset_server.load("bell.png"),
            cogwheel: asset_server.load("cogwheel.png"),
        }
    }
}

fn main() {
    App::new()
        .init_resource::<UiState>()
        .init_resource::<WindowState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_startup_system(configure_visuals)
        .add_system(ui_example)
        .add_system(sf_forms_windows)
        .run();
}

#[derive(Default)]
struct UiState {
    label: String,
    account_create: bool,
    account_seeded: bool,
    account_fund: bool,
    account_exists: bool,
    account_balance: bool,
    asset_create_class: bool,
    asset_class_info: bool,
    asset_create: bool,
    asset_info: bool,
    asset_update_metadata: bool,
    asset_mint: bool,
    asset_burn: bool,
    asset_balance: bool,
    asset_transfer_from: bool,
    bag_register: bool,
    bag_create: bool,
    bag_sweep: bool,
    bag_deposit: bool,
    bundle_register: bool,
    bundle_mint: bool,
    bundle_burn: bool,
    market_create: bool,
    market_create_market_rate: bool,
    market_deposit_assets: bool,
    market_exchange_assets: bool
}

#[derive(Default)]
struct WindowState {
    is_open: bool
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
    egui_ctx.ctx_mut().set_visuals(egui::Visuals::light());
}

fn ui_example(
    mut egui_ctx: ResMut<EguiContext>, 
    mut ui_state: ResMut<UiState>,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut rendered_texture_id2: Local<egui::TextureId>,
    mut rendered_texture_id3: Local<egui::TextureId>,
    mut rendered_texture_id4: Local<egui::TextureId>,
    mut rendered_texture_id5: Local<egui::TextureId>,
    mut is_initialized: Local<bool>, 
    images: Local<Images>,
) {
    if !*is_initialized {
        *is_initialized = true;
        *rendered_texture_id = egui_ctx.add_image(images.sugarfunge_logo.clone_weak());
        *rendered_texture_id2 = egui_ctx.add_image(images.avatar.clone_weak());
        *rendered_texture_id3 = egui_ctx.add_image(images.mail.clone_weak());
        *rendered_texture_id4 = egui_ctx.add_image(images.bell.clone_weak());
        *rendered_texture_id5 = egui_ctx.add_image(images.cogwheel.clone_weak());
    }

    egui::SidePanel::left("left_side_panel")
    .default_width(200.0)
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(42.5);

            ui.add(egui::widgets::Image::new(
                *rendered_texture_id,
                [100.0, 62.0],
            ));
        });

        ui.separator();

        egui::menu::bar(ui, |ui| {
            ui.vertical(|ui| {
                egui::menu::menu_button(ui, "Account", |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.vertical(|ui| {
                            if ui.button("Create account").clicked() {
                                ui_state.account_create = true
                            }

                            ui.separator();
                            
                            if ui.button("Account seeded").clicked() {
                                ui_state.account_seeded = true
                            }

                            ui.separator();

                            if ui.button("Fund account").clicked() {
                                ui_state.account_fund = true
                            }

                            ui.separator();

                            if ui.button("Account exists").clicked() {
                                ui_state.account_exists = true
                            }

                            ui.separator();

                            if ui.button("Account balance").clicked() {
                                ui_state.account_balance = true
                            }

                        })
                    });
                });

                ui.separator();

                egui::menu::menu_button(ui, "Asset", |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.vertical(|ui| {
                            if ui.button("Create asset class").clicked() {
                                ui_state.asset_create_class = true
                            }

                            ui.separator();
                            
                            if ui.button("Asset class info").clicked() {
                                ui_state.asset_class_info = true
                            }

                            ui.separator();

                            if ui.button("Create asset").clicked() {
                                ui_state.asset_create = true
                            }

                            ui.separator();

                            if ui.button("Asset info").clicked() {
                                ui_state.asset_info = true
                            }

                            ui.separator();

                            if ui.button("Update asset metadata").clicked() {
                                ui_state.asset_update_metadata = true
                            }

                            ui.separator();

                            if ui.button("Mint asset").clicked() {
                                ui_state.asset_mint = true
                            }

                            ui.separator();

                            if ui.button("Burn asset").clicked() {
                                ui_state.asset_burn = true
                            }

                            ui.separator();

                            if ui.button("Asset balance").clicked() {
                                ui_state.asset_balance = true
                            }

                            ui.separator();

                            if ui.button("Transfer asset").clicked() {
                                ui_state.asset_transfer_from = true
                            }
                        })
                    });
                });

                ui.separator();

                egui::menu::menu_button(ui, "Bag", |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.vertical(|ui| {
                            if ui.button("Register bag").clicked() {
                                ui_state.bag_register= true
                            }

                            ui.separator();
                            
                            if ui.button("Create bag").clicked() {
                                ui_state.bag_create = true
                            }

                            ui.separator();

                            if ui.button("Deposit bag").clicked() {
                                ui_state.bag_deposit = true
                            }

                            ui.separator();

                            if ui.button("Sweep bag").clicked() {
                                ui_state.bag_sweep = true
                            }
                        })
                    });
                });

                ui.separator();

                egui::menu::menu_button(ui, "Bundle", |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.vertical(|ui| {
                            if ui.button("Register bundle").clicked() {
                                ui_state.bundle_register = true
                            }

                            ui.separator();
                            
                            if ui.button("Mint bundle").clicked() {
                                ui_state.bundle_mint = true
                            }

                            ui.separator();

                            if ui.button("Burn bundle").clicked() {
                                ui_state.bundle_burn = true
                            }
                        })
                    });
                });

                ui.separator();

                egui::menu::menu_button(ui, "Market", |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.vertical(|ui| {
                            if ui.button("Create market").clicked() {
                                ui_state.market_create = true
                            }

                            ui.separator();
                            
                            if ui.button("Create market rate").clicked() {
                                ui_state.market_create_market_rate = true
                            }

                            ui.separator();
                        
                            if ui.button("Deposit assets").clicked() {
                                ui_state.market_deposit_assets = true
                            }

                            ui.separator();
                            
                            if ui.button("Exchange assets").clicked() {
                                ui_state.market_exchange_assets = true
                            } 
                        })
                    });
                });
            });
        })
    });

    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);

                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("SEARCH ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });
                
                ui.add_space(420.0);
    
                ui.horizontal(|ui| {
                    ui.add(egui::widgets::Image::new(
                        *rendered_texture_id2,
                        [60.0, 30.0],
                    ));

                    ui.add_space(3.0);
    
                    ui.label("Welcome, Favorite User");

                    ui.add_space(28.5);
    
                    ui.separator();

                    ui.add_space(5.0);

                    ui.add(egui::widgets::Image::new(
                        *rendered_texture_id3,
                        [25.0, 25.0],
                    ));

                    ui.add_space(5.0);
    
                    ui.separator();
    
                    ui.add_space(5.0);

                    ui.add(egui::widgets::Image::new(
                        *rendered_texture_id4,
                        [25.0, 25.0],
                    ));
    
                    ui.add_space(5.0);

                    ui.separator();
    
                    ui.add_space(5.0);

                    ui.add(egui::widgets::Image::new(
                        *rendered_texture_id5,
                        [25.0, 27.0],
                    ));

                    ui.add_space(5.0);
    
                    ui.separator();


                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.add_space(4.0);
                            ui.button(RichText::new("Logout").color(Color32::RED))
                      
                        }
                    );
    
    
                });
            });

            ui.add_space(2.0);
            
        });
    });

    egui::SidePanel::right("right_side_panel")
    .default_width(200.0)
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(42.5);

            ui.add(egui::widgets::Image::new(
                *rendered_texture_id,
                [100.0, 62.0],
            ));
        });

        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                1.0,
                Color32::GREEN
            );

            ui.label("Balance");

            ui.with_layout(
                egui::Layout::top_down(egui::Align::RIGHT),
                |ui| {
                    ui.colored_label(Color32::BLACK, "$14,679");              
                }
            );
        });

        ui.add_space(15.0);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    1.0,
                    Color32::GRAY
                );
                ui.colored_label(Color32::WHITE, "Quick links");
            });

            ui.add_space(7.0);

            ui.label("New tasks");

            ui.separator();

            ui.label("Activity");
            
            ui.separator();
            
            ui.label("Parameters");
            
            ui.separator();

            ui.label("My cards");
            
            ui.separator();

            ui.label("Users");

        });

        ui.add_space(15.0);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                1.0,
                Color32::from_rgb(230, 92, 0)
                );
                ui.colored_label(Color32::WHITE,"Customer support?");
            });

            ui.vertical(|ui| {
                ui.add_space(15.0);
                ui.label("Name");
            });

            ui.vertical(|ui| {
                ui.add_space(12.0);
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.vertical(|ui| {
                ui.add_space(15.0);
                ui.label("Email");
            });

            ui.vertical(|ui| {
                ui.add_space(12.0);
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.vertical(|ui| {
                ui.add_space(15.0);
                ui.label("Comments");
            });

            ui.vertical(|ui| {
                ui.add_space(12.0);
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.add_space(140.0);        
                ui.button(RichText::new("Send").color(Color32::from_rgb(230, 92, 0)))
            })

        });
    });

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            1.0,
            Color32::DARK_GRAY
            );

            ui.label(RichText::new("Recent activity").color(Color32::WHITE).heading());

            ui.with_layout(
                egui::Layout::top_down(egui::Align::RIGHT),
                |ui| {
                    ui.label(RichText::new("See All >").color(Color32::WHITE).heading());            
              
                }
            );
        });

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Green dot POS Money Load").color(Color32::BLACK).heading());
                ui.label("Funds loaded through Green Dot load network.");                
            });

            // ui.add_space(530.0);

            // ui.vertical(|ui| {
            //     ui.colored_label(Color32::GREEN, "$834.19");
            //     ui.label("05/01/2022");                
            // });

            ui.with_layout(
                egui::Layout::top_down(egui::Align::RIGHT),
                |ui| {
                ui.colored_label(Color32::GREEN, "$834.19");
                ui.label("05/01/2022");             
              
                }
            );
            
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("POS Signature Purchase").color(Color32::BLACK).heading());
                ui.label("SMOKERS WORLD II, LAS VEGAS, NV, USA");                
            });

            // ui.add_space(560.0);

            // ui.vertical(|ui| {
            //     ui.colored_label(Color32::RED, "$450.39");
            //     ui.label("05/04/2022");                
            // });

            ui.with_layout(
                egui::Layout::top_down(egui::Align::RIGHT),
                |ui| {
                    ui.colored_label(Color32::RED, "$450.39");
                    ui.label("05/04/2022");           
              
                }
            );
        });
        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Load Money").color(Color32::BLACK).heading());
                ui.label("Funds transferred from Bank Account *5075");                
            });

            // ui.add_space(542.5);

            // ui.vertical(|ui| {
            //     ui.colored_label(Color32::GREEN, "$188.17");
            //     ui.label("05/06/2022");                
            // });

            ui.with_layout(
                egui::Layout::top_down(egui::Align::RIGHT),
                |ui| {
                    ui.colored_label(Color32::GREEN, "$188.17");
                    ui.label("05/06/2022");             
              
                }
            );
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("POS Signature Purchase").color(Color32::BLACK).heading());
                ui.label("VONS STORE, 2511, HENDERSON, NY, USA");                
            });

            // ui.add_space(550.0);

            // ui.vertical(|ui| {
            //     ui.colored_label(Color32::RED,"$7.19");
            //     ui.label("05/14/2022");                
            // });

            ui.with_layout(
                egui::Layout::top_down(egui::Align::RIGHT),
                |ui| {
                    ui.colored_label(Color32::RED,"$7.19");
                    ui.label("05/14/2022");              
              
                }
            );
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Load Money").color(Color32::BLACK).heading());
                ui.label("Funds transferred from Bank Account *5075");                
            });

            // ui.add_space(542.5);

            // ui.vertical(|ui| {
            //     ui.colored_label(Color32::GREEN, "$188.17");
            //     ui.label("05/15/2022");                
            // });


            ui.with_layout(
                egui::Layout::top_down(egui::Align::RIGHT),
                |ui| {
                    ui.colored_label(Color32::GREEN, "$188.17");
                    ui.label("05/15/2022");             
              
                }
            );
        });

        ui.separator();
    });

}

fn sf_forms_windows(
    mut egui_ctx: ResMut<EguiContext>, 
    mut ui_state: ResMut<UiState>,
    mut window_state: ResMut<WindowState>
) {
    if ui_state.account_create {
        egui::Window::new("Create account")
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Account created!");
        });
    }

    if ui_state.account_seeded {
        let mut is_open: bool = true;
        let mut close = false;
        egui::Window::new("Account seeded").open(&mut is_open)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });
                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        close = true
                    }
                })
            });
        });

        is_open &= !close;


    }

    if ui_state.account_fund {
        egui::Window::new("Fund account").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amount: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("To: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.account_exists {
        egui::Window::new("Account exists").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Account: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.account_balance {
        egui::Window::new("Account balance").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Account: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_create_class {
        egui::Window::new("Create asset class").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Metadata: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Owner: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_class_info {
        egui::Window::new("Asset class info").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_create {
        egui::Window::new("Create asset").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });
                
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Account: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });  

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Metadata: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_info {
        egui::Window::new("Asset info").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_update_metadata {
        egui::Window::new("Update asset metadata").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Metadata: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_mint {
        egui::Window::new("Mint asset").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("To: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amount: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_burn {
        egui::Window::new("Burn asset").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("From: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amount: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_balance {
        egui::Window::new("Asset balance").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Account: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.asset_transfer_from {
        egui::Window::new("Transfer asset").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("From: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("To: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amount: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.bag_register {
        egui::Window::new("Register bag").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Metadata: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.bag_create {
        egui::Window::new("Create bag").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Owners: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Shares: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.bag_deposit {
        egui::Window::new("Deposit bag").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Bag: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class ids: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset ids: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amounts: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.bag_sweep {
        egui::Window::new("Sweep bag").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("To: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Bag: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset ids: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amounts: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.bundle_register {
        egui::Window::new("Register bundle").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Metadata: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Schema: ");
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Class ids: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Asset ids: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amounts: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.bundle_mint {
        egui::Window::new("Mint bundle").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("From: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("To: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Bundle id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amount: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.bundle_burn {
        egui::Window::new("Burn bundle").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("From: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("To: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Bundle id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Amount: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }

    if ui_state.market_create {
        egui::Window::new("Create market").show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Seed: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.label("Market id: ");
                    });
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.text_edit_singleline(&mut ui_state.label);
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(285.0);
                    if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
                    }

                })
            });
        });
    }
}