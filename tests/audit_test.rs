// tests/audit_test.rs

mod audit {
    mod test_format_vulnerability_diagnostics;
    mod test_manage_osv_cache;
    mod test_query_osv_api;

    mod parsers {
        mod javascript {
            mod test_bun;
            mod test_npm;
            mod test_package_json;
            mod test_pnpm;
            mod test_yarn;
        }
        mod python {
            mod test_pip;
            mod test_poetry;
            mod test_uv;
        }
        mod rust {
            mod test_cargo;
        }
    }

    mod remediation {
        pub mod test_execute_package_manager_upgrade;
        pub mod test_orchestrate_vulnerability_remediation;
        pub mod test_resolve_remediation_context;
        pub mod test_verify_remediation_success;
    }
}
