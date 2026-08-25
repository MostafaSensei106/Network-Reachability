import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:network_reachability_example/main.dart';

void main() {
  testWidgets('NetworkEngineHub smoke test', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: NetworkEngineHub(),
      ),
    );
    expect(find.byType(NetworkEngineHub), findsOneWidget);
  });
}
