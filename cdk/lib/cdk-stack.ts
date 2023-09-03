import * as cdk from 'aws-cdk-lib';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as path from 'path';
//import * as apigatewayv2 from 'aws-cdk-lib/aws-apigatewayv2';
import {
  //CorsHttpMethod,
  HttpApi,
  HttpMethod,
} from '@aws-cdk/aws-apigatewayv2-alpha';
import { HttpLambdaIntegration } from '@aws-cdk/aws-apigatewayv2-integrations-alpha';
import { Construct } from 'constructs';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Lambda function
    const helloLambda = new lambda.Function(this, 'RustHelloLambda', {
      code: lambda.Code.fromAsset(path.join(__dirname, '../../rust_lambda/target/lambda/hello_lambda')),
      handler: 'not.used.in.rust',
      runtime: lambda.Runtime.PROVIDED_AL2,
      memorySize: 512,
      timeout: cdk.Duration.seconds(10),
      environment: {
        RUST_BACKTRACE: '1',
        TZ: 'Asia/Tokyo',
      },
    });

    const httpApi = new HttpApi(this, 'http-api-rust', {
      description: 'Http API for Rust Lambda',
    });
    
    httpApi.addRoutes({
      path: '/',
      methods: [HttpMethod.GET],
      integration: new HttpLambdaIntegration(
        'rust-hello-lambda',
        helloLambda,
      )

    });

    new cdk.CfnOutput(this, "HttpApiUrl", {
      value: httpApi.url!,
    });
  }
}
