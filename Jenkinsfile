pipeline {
    agent { 
        kubernetes { 
            label 'qlogic-rs'
            defaultContainer 'rust'
            yaml """
apiVersion: v1
kind: Pod
metadata:
  name: jenkins-ci
spec:
  serviceAccountName: default
  containers:
    - name: rust
      image: rust
"""
        } 
    }
    stages {
        stage('build') {
            steps {
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
