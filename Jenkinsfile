pipeline {
    agent { docker { image 'rust' } }
    triggers {
        cron('H 2 * * *')
    }
    stages {
        stage('build') {
            steps {
                sh 'cargo check'
                sh 'cargo build --release'
            }
        }
        stage('test') {
            steps {
                sh 'cargo test --all'
            }
        }
    }  
}
